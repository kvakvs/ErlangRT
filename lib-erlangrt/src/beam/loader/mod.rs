//! Code loader for BEAM files uses 3 stage approach.
//! Stage 1 reads the BEAM file and fills the loader state structure.
//! Stage 2 commits changes to the VM (atom table for example)
//! Stage 3 (finalize) returns Erlang module object ready for code server.
//!
//! Call `let l = Loader::new()`, then `l.load(filename)`, then
//! `l.load_stage2(&mut vm)` and finally `let modp = l.load_finalize()`
mod beam_file;
mod compact_term;
mod impl_fix_labels;
mod impl_parse_code;
mod impl_setup_imports;
mod impl_stage2;
mod load_time_structs;
mod load_time_term;

use crate::{
  beam::loader::{beam_file::BeamFile, load_time_term::LtTerm},
  defs::Word,
  emulator::{
    code::{opcode::RawOpcode, Code, CodeOffset, LabelId},
    code_srv::CodeServer,
    function::FunEntry,
    module::{self, Module, VersionedModuleName},
  },
  fail::RtResult,
  term::lterm::*,
};
use core::mem;
use std::{collections::BTreeMap, path::PathBuf};

// macro_rules! rt_debug {
//    ($($arg:tt)*) => (if cfg!(trace_beam_loader) { println!($($arg)*); })
//}

fn module() -> &'static str {
  "beam/loader: "
}

/// Errors created when parsing compact term format. They are delivered to the
/// end caller wrapped in `fail::Error:CodeLoadingCompactTerm(x)`
#[derive(Debug)]
pub enum CTError {
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
  PatchCodeOffset(Word),
  PatchJtabElement(LTerm, Word),
}

/// BEAM loader state.
struct LoaderState {
  beam_file: BeamFile,

  name: Option<VersionedModuleName>,

  //--- Stage 2 structures filled later ---
  /// Atoms converted to VM terms. Remember to use from_loadtime_atom_index()
  /// which will deduce 1 from the index automatically
  vm_atoms: Vec<LTerm>,
  // vm_funs: BTreeMap<FunArity, CodeOffset>,

  //--- Code postprocessing and creating a function object ---
  /// Accumulate code for the current function here then move it when done.
  code: Code,

  /// Labels are stored here while loading, for later resolve.
  /// Type:: map<Label, Offset>
  labels: BTreeMap<LabelId, CodeOffset>,

  /// Locations of label values are collected and at a later pass replaced
  /// with their word values or function pointer (if the label points outside)
  replace_labels: Vec<PatchLocation>,

  funs: module::ModuleFunTable,

  /// Raw imports transformed into 3 tuples {M,Fun,Arity} and stored on lit heap
  imports: Vec<LTerm>,

  lambdas: Vec<FunEntry>,
  /*  /// A map of F/Arity -> HOExport which uses literal heap but those created
   *  /// during runtime will be using process heap.
   *  exports: BTreeMap<FunArity, LTerm> */
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
  fn atom_from_loadtime_index(&self, n: usize) -> LTerm {
    if n == 0 {
      return LTerm::nil();
    }
    self.vm_atoms[n as usize - 1]
  }

  fn module_name(&self) -> LTerm {
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
  fn create_jump_destination(&self, dst_offset: CodeOffset) -> Word {
    let CodeOffset(offs) = dst_offset;
    let ptr = &self.code[offs] as *const Word;
    LTerm::make_cp(ptr).raw()
  }

  /// Given a load-time `Atom_` or a structure possibly containing `Atom_`s,
  /// resolve it to a runtime atom index using a lookup table.
  pub fn resolve_loadtime_values(&self, arg: &LtTerm) -> Option<LtTerm> {
    match *arg {
      // A special value 0 means NIL []
      LtTerm::LoadtimeAtom(0) => Some(LtTerm::Nil),

      // Repack load-time atom via an `LTerm` index into an `FTerm` atom
      LtTerm::LoadtimeAtom(i) => {
        let aindex = self.atom_from_loadtime_index(i).atom_index();
        Some(LtTerm::Atom(aindex))
      }

      // ExtList_ can contain Atom_ - convert them to runtime Atoms
      LtTerm::LoadtimeExtlist(ref lst) => {
        let mut result: Vec<LtTerm> = Vec::new();
        result.reserve(lst.len());
        for x in lst.iter() {
          match self.resolve_loadtime_values(x) {
            Some(tmp) => result.push(tmp),
            None => result.push(x.clone()),
          }
        }
        Some(LtTerm::LoadtimeExtlist(result))
      }
      // Otherwise no changes
      _ => None,
    }
  }
}

/// Report a bad opcode arg
// TODO: Use this more, than just label opcode
fn op_badarg_panic(op: RawOpcode, args: &[LtTerm], argi: Word) {
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
  println!("BEAM loader: from {}", mod_file_path.to_str().unwrap());

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
