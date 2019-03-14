//! Code loader for BEAM files uses 3 stage approach.
//! Stage 1 reads the BEAM file and fills the loader state structure.
//! Stage 2 commits changes to the VM (atom table for example)
//! Stage 3 (finalize) returns Erlang module object ready for code server.
//!
//! Call `let l = Loader::new()`, then `l.load(filename)`, then
//! `l.load_stage2(&mut vm)` and finally `let modp = l.load_finalize()`
#[macro_use]
mod macros;

mod beam_file;
mod compact_term;
mod impl_fix_labels;
mod impl_parse_code;
mod impl_setup_imports;
mod impl_stage2;
mod load_time_structs;

use crate::{
  beam::loader::beam_file::BeamFile,
  defs::Word,
  emulator::{
    code::{opcode::RawOpcode, Code, CodeOffset},
    code_srv::CodeServer,
    function::FunEntry,
    module::{self, Module, VersionedModuleName},
  },
  fail::RtResult,
  term::{
    boxed::{self, boxtype::BOXTYPETAG_JUMP_TABLE},
    value::{self, *},
  },
};
use core::mem;
use std::{collections::BTreeMap, path::PathBuf};

#[inline]
const fn module() -> &'static str {
  "loader: "
}

/// Errors created when parsing compact term format. They are delivered to the
/// end caller wrapped in `fail::Error:CodeLoadingCompactTerm(x)`
#[derive(Debug)]
pub enum CompactTermError {
  BadLiteralTag,
  BadAtomTag,
  BadXRegTag,
  BadYRegTag,
  BadLabelTag,
  BadCharacterTag,
  BadIntegerTag,
  BadExtendedTag(String),
}

/// Represents an instruction to patch either a code location or an index in
/// a tuple which represents a jump table (pairs value -> label)
enum PatchLocation {
  PatchCodeOffset(usize),
  PatchJumpTable(Term),
}

/// BEAM loader state.
struct LoaderState {
  beam_file: BeamFile,

  name: Option<VersionedModuleName>,

  //--- Stage 2 structures filled later ---
  /// Atoms converted to VM terms. Remember to use from_loadtime_atom_index()
  /// which will deduce 1 from the index automatically
  vm_atoms: Vec<Term>,

  //--- Code postprocessing and creating a function object ---
  /// Accumulate code for the current function here then move it when done.
  code: Code,

  /// Labels are stored here while loading, for later resolve.
  /// Type:: map<Label_id: usize, Offset>
  labels: BTreeMap<usize, CodeOffset>,

  /// Locations of label values are collected and at a later pass replaced
  /// with their word values or function pointer (if the label points outside)
  replace_labels: Vec<PatchLocation>,

  funs: module::ModuleFunTable,

  /// Raw imports transformed into 3 tuples {M,Fun,Arity} and stored on lit heap
  imports: Vec<Term>,

  lambdas: Vec<FunEntry>,
}

impl LoaderState {
  /// Construct a new loader state.
  pub fn new(beam_file: BeamFile) -> LoaderState {
    LoaderState {
      beam_file,
      name: None,

      vm_atoms: Vec::new(),

      code: Vec::new(),
      labels: BTreeMap::new(),
      replace_labels: Vec::new(),
      funs: BTreeMap::new(),
      imports: Vec::new(),
      lambdas: Vec::new(),
      // exports: BTreeMap::new(),
    }
  }

  /// With atom index loaded from BEAM query `self.vm_atoms` array. Takes into
  /// account special value 0 and offsets the index down by 1.
  fn atom_from_loadtime_index(&self, n: usize) -> Term {
    if n == 0 {
      return Term::nil();
    }
    self.vm_atoms[n as usize - 1]
  }

  fn module_name(&self) -> Term {
    match &self.name {
      Some(mod_id) => mod_id.module,
      None => panic!("{}mod_id must be set at this point", module()),
    }
  }

  /// At this point loading is finished, and we create Erlang module and
  /// return a reference counted pointer to it. VM (the caller) is responsible
  /// for adding the module to its code registry.
  pub fn load_finalize(&mut self) -> RtResult<Box<Module>> {
    let mut newmod = match &self.name {
      Some(mod_id) => Box::new(module::Module::new(mod_id)),
      None => panic!("{}mod_id must be set at this point", module()),
    };

    // Move funs into new module
    {
      mem::swap(&mut self.funs, &mut newmod.funs);
      mem::swap(&mut self.code, &mut newmod.code);
      mem::swap(&mut self.beam_file.lit_heap, &mut newmod.lit_heap);
      mem::swap(&mut self.lambdas, &mut newmod.lambdas);
    }

    Ok(newmod)
  }

  //============================================================================

  fn set_mod_id(&mut self, code_server: &mut CodeServer) {
    assert!(!self.vm_atoms.is_empty());
    // 0-th atom in the atom table is module name
    let mod_name = self.vm_atoms[0];
    self.name = Some(VersionedModuleName {
      module: mod_name,
      version: code_server.next_module_version(mod_name),
    });
  }

  /// Given label destination and `self.code` length calculate a relative
  /// signed jump offset for it.
  fn create_jump_destination(&self, dst_offset: CodeOffset) -> Term {
    let CodeOffset(offs) = dst_offset;
    let ptr = &self.code[offs] as *const Word;
    Term::make_cp(ptr)
  }

  /// Given a value, possibly a load-time value or a structure possibly
  /// containing nested load-time values, resolve it using lookup tables.
  pub fn resolve_value(&self, arg: Term) -> Term {
    if arg.is_loadtime() {
      let lt_tag = arg.get_loadtime_tag();
      let lt_val = arg.get_loadtime_val();

      if lt_tag == value::SPECIAL_LT_ATOM {
        // A special value 0 means NIL []
        if lt_val == 0 {
          return Term::nil();
        }

        // Convert loadtime atom into VM atom using the lookup table
        self.atom_from_loadtime_index(lt_val)
      } else {
        arg // no change
      }
    } else if arg.is_boxed_of_type(BOXTYPETAG_JUMP_TABLE) {
      // TODO: Generic iteration through any container boxed?
      let lst = arg.get_box_ptr_mut::<boxed::JumpTable>();
      unsafe {
        (*lst).inplace_map_t(|_, val| self.resolve_value(val));
      }
      arg
    } else {
      // Otherwise no changes
      arg
    }
  }
}

/// Report a bad opcode arg
// TODO: Use this more, than just label opcode
fn op_badarg_panic(op: RawOpcode, args: &[Term], argi: Word) {
  panic!(
    "{}Opcode {} the arg #{} in {:?} is bad",
    module(),
    op.get(),
    argi,
    args
  )
}

pub fn load_module(
  code_srv: &mut CodeServer,
  mod_file_path: &PathBuf,
) -> RtResult<Box<Module>> {
  rtdbg!("BEAM loader: from {}", mod_file_path.to_str().unwrap());

  // Preload data structures
  // located in impl_read_chunks.rs
  let beam_file = BeamFile::read_chunks(mod_file_path)?;
  let mut loader = LoaderState::new(beam_file);

  // Apply changes to the VM after module loading succeeded. The
  // module object is not created yet, but some effects like atoms table
  // we can already apply.
  loader.stage2_register_atoms(code_srv);
  loader.stage2_fill_lambdas();

  // located in impl_parse_code.rs
  loader.parse_raw_code()?;

  // located in impl_fix_labels.rs
  loader.fix_labels()?;

  // located in impl_setup_imports.rs
  loader.setup_imports()?;

  loader.load_finalize()
}
