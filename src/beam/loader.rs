//!
//! Code loader for BEAM files uses 3 stage approach.
//! Stage 1 reads the BEAM file and fills the loader state structure.
//! Stage 2 commits changes to the VM (atom table for example)
//! Stage 3 (finalize) returns Erlang module object ready for code server.
//!
//! Call `let l = Loader::new()`, then `l.load(filename)`, then
//! `l.load_stage2(&mut vm)` and finally `let modp = l.load_finalize()`
//!
use bytes::Bytes;
use std::path::PathBuf;
use std::collections::BTreeMap;
use std::mem;
use std::io::{Read, Cursor};
use compress::zlib;

use term::boxed;
use beam::compact_term;
use beam::gen_op;
use bif;
use fail::{Hopefully, Error};
use rt_defs::{Word, Arity};
use rt_util::bin_reader::{BinaryReader, ReadError};
use rt_util::ext_term_format as etf;

use emulator::atom;
use emulator::code::pointer::CodePtrMut;
use emulator::code::{LabelId, CodeOffset, Code, opcode};
use emulator::code;
use emulator::code_srv::CodeServer;
use emulator::code_srv::module_id::VersionedModuleId;
use emulator::funarity::FunArity;
use emulator::function::{FunEntry};
use emulator::heap::{Heap, DEFAULT_LIT_HEAP};
use emulator::mfa::MFArity;
use emulator::module;

use term::fterm::FTerm;
use term::lterm::*;
use term::raw::*;
use term::term_builder::TermBuilder;


pub fn module() -> &'static str { "beam::loader: " }


/// Imports table item (mfa's referred by this module).
/// Raw data structure as loaded from BEAM file
#[allow(dead_code)]
struct LImport {
  mod_atom_i: u32,
  fun_atom_i: u32,
  arity: Arity,
}


/// Exports table item, as specified in `-export()` attribute.
/// Raw data structure as loaded from BEAM file.
#[allow(dead_code)]
struct LExport {
  fun_atom_i: u32,
  arity: Arity,
  label: u32,
}


/// Function closures used in this file, with info on captured values.
/// Raw data structure as loaded from BEAM file
#[allow(dead_code)]
struct LFun {
  arity: Arity,
  fun_atom_i: u32,
  code_pos: u32,
  index: u32,
  nfree: u32,
  ouniq: u32,
}


/// Represents an instruction to patch either a code location or an index in
/// a tuple which represents a jump table (pairs value -> label)
enum PatchLocation {
  PatchCodeOffset(Word),
  PatchJtabElement(LTerm, Word)
}


/// Stage 1 raw structures, as loaded and decoded from the beam file but not
/// ready to be used in runtime
struct LoaderRaw {
  /// Raw atoms loaded from BEAM module as strings
  atoms: Vec<String>,
  imports: Vec<LImport>,
  exports: Vec<LExport>,
  locals: Vec<LExport>,
  lambdas: Vec<LFun>,
  /// Temporary storage for loaded code, will be parsed in stage 2
  code: Vec<u8>,
}


impl LoaderRaw {
  fn new() -> LoaderRaw {
    LoaderRaw {
      atoms: Vec::new(),
      imports: Vec::new(),
      exports: Vec::new(),
      locals: Vec::new(),
      lambdas: Vec::new(),
      code: Vec::new(),
    }
  }
}


/// BEAM loader state.
pub struct Loader {
  mod_id: Option<VersionedModuleId>,
  raw: LoaderRaw,

  //--- Stage 2 structures filled later ---
  /// Atoms converted to VM terms. Remember to use from_loadtime_atom_index()
  /// which will deduce 1 from the index automatically
  vm_atoms: Vec<LTerm>,
  //vm_funs: BTreeMap<FunArity, CodeOffset>,

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

  /// Literal table decoded into friendly terms (does not use process heap).
  lit_tab: Vec<LTerm>,

  /// A place to allocate larger lterms (literal heap)
  lit_heap: Heap,

  /// Proplist of module attributes as loaded from "Attr" section
  mod_attrs: LTerm,

  /// Compiler flags as loaded from "Attr" section
  compiler_info: LTerm,

  /// Raw imports transformed into 3 tuples {M,Fun,Arity} and stored on lit heap
  imports: Vec<LTerm>,

  lambdas: Vec<FunEntry>,

//  /// A map of F/Arity -> HOExport which uses literal heap but those created
//  /// during runtime will be using process heap.
//  exports: BTreeMap<FunArity, LTerm>
}


impl Loader {
  /// Construct a new loader state.
  pub fn new() -> Loader {
    Loader {
      raw: LoaderRaw::new(),
      mod_id: None,

      lit_tab: Vec::new(),
      lit_heap: Heap::new(DEFAULT_LIT_HEAP),
      vm_atoms: Vec::new(),

      code: Vec::new(),
      labels: BTreeMap::new(),
      replace_labels: Vec::new(),
      funs: BTreeMap::new(),
      mod_attrs: LTerm::nil(),
      compiler_info: LTerm::nil(),
      imports: Vec::new(),
      lambdas: Vec::new(),
      //exports: BTreeMap::new(),
    }
  }


  /// With atom index loaded from BEAM query `self.vm_atoms` array. Takes into
  /// account special value 0 and offsets the index down by 1.
  fn atom_from_loadtime_index(&self, n: u32) -> LTerm {
    if n == 0 { return LTerm::nil() }
    self.vm_atoms[n as usize - 1]
  }


  fn module_name(&self) -> LTerm {
    match self.mod_id {
      Some(mod_id) => mod_id.module(),
      None => panic!("{}mod_id must be set at this point", module()),
    }
  }


  /// Loading the module. Validate the header and iterate over sections,
  /// then call `load_stage2()` to apply changes to the VM, and then finalize
  /// it by calling `load_finalize()` which will return you a module object.
  pub fn load(&mut self, fname: &PathBuf) -> Hopefully<()> {
    // Prebuffered BEAM file should be released as soon as the initial phase
    // is done. TODO: [Performance] Use memmapped file?
    let mut r = BinaryReader::from_file(fname);

    // Parse header and check file FOR1 signature
    let hdr1 = Bytes::from(&b"FOR1"[..]);
    r.ensure_bytes(&hdr1)?;

    let _beam_sz = r.read_u32be();

    // Check BEAM signature
    let hdr2 = Bytes::from(&b"BEAM"[..]);
    r.ensure_bytes(&hdr2)?;

    loop {
      // EOF may strike here when we finished reading
      let chunk_h = match r.read_str_latin1(4) {
        Ok(s) => s,
        // EOF is not an error
        Err(ReadError::PrematureEOF) => break,
        Err(e) => return Err(Error::CodeLoading(e))
      };
      let chunk_sz = r.read_u32be();
      let pos_begin = r.pos();

      //println!("Chunk {}", chunk_h);
      match chunk_h.as_ref() {
        "Atom" => self.load_atoms_latin1(&mut r),
        "Attr" => self.load_attributes(&mut r)?,
        "AtU8" => self.load_atoms_utf8(&mut r),
        "CInf" => self.load_compiler_info(&mut r)?,
        "Code" => self.load_code(&mut r, chunk_sz as Word),
        "ExpT" => self.raw.exports = self.load_exports(&mut r),
        "FunT" => self.load_fun_table(&mut r),
        "ImpT" => self.load_imports(&mut r),
        "Line" => self.load_line_info(&mut r),
        "LitT" => self.load_literals(&mut r, chunk_sz as Word),
        // LocT same format as ExpT, but for local functions
        "LocT" => self.raw.locals = self.load_exports(&mut r),

        "Dbgi" | // skip debug info
        "StrT" | // skip strings TODO load strings?
        "Abst" => r.skip(chunk_sz as Word), // skip abstract code

        other => {
          let msg = format!("{}Unexpected chunk: {}", module(), other);
          return Err(Error::CodeLoadingFailed(msg))
        }
      }

      // The next chunk is aligned at 4 bytes
      let aligned_sz = 4 * ((chunk_sz + 3) / 4);
      r.seek(pos_begin + aligned_sz as Word);
    }

    Ok(())
  }


  fn stage2_register_atoms(&mut self, code_server: &mut CodeServer) {
    self.vm_atoms.reserve(self.raw.atoms.len());
    for a in &self.raw.atoms {
      self.vm_atoms.push(atom::from_str(a));
    }

    // Create a new version number for this module and fill self.mod_id
    self.set_mod_id(code_server)
  }


  fn stage2_fill_lambdas(&mut self) {
    // Convert LFuns in self.raw.funs to FunEntries
    for rf in &self.raw.lambdas {
      let fun_name = self.atom_from_loadtime_index(rf.fun_atom_i);
      let mfa = MFArity::new(self.module_name(), fun_name, rf.arity);
      println!("{}stage2_fill_lambdas mfa={}", module(), mfa);
      self.lambdas.push(FunEntry::new(mfa, rf.nfree))
    }
  }


  /// Call this to apply changes to the VM after module loading succeeded. The
  /// module object is not created yet, but some effects like atoms table
  /// we can already apply.
  pub fn load_stage2(&mut self, code_server: &mut CodeServer) -> Hopefully<()> {
    self.stage2_register_atoms(code_server);
    self.stage2_fill_lambdas();

    self.postprocess_parse_raw_code()?;
    //unsafe { disasm::disasm(self.code.as_slice(), None) }
    self.postprocess_fix_labels();
    self.postprocess_setup_imports()?;

    Ok(())
  }


  /// At this point loading is finished, and we create Erlang module and
  /// return a reference counted pointer to it. VM (the caller) is responsible
  /// for adding the module to its code registry.
  pub fn load_finalize(&mut self) -> Hopefully<module::Ptr> {
    let mut newmod = match self.mod_id {
      Some(mod_id) => module::Module::new(&mod_id),
      None => panic!("{}mod_id must be set at this point", module()),
    };

    // Move funs into new module
    {
      mem::swap(&mut self.funs, &mut newmod.funs);
      mem::swap(&mut self.code, &mut newmod.code);
      mem::swap(&mut self.lit_heap, &mut newmod.lit_heap);
      mem::swap(&mut self.lambdas, &mut newmod.lambdas);

//      unsafe {
//        disasm::disasm(&mod1.code, None);
//        //mod1.lit_heap.dump()
//      };
    }

    Ok(newmod)
  }

  //============================================================================

  /// Read Attr section: two terms (module attributes and compiler info) encoded
  /// as external term format.
  fn load_attributes(&mut self, r: &mut BinaryReader) -> Hopefully<()> {
    let mut tb = TermBuilder::new(&mut self.lit_heap);
    self.mod_attrs = etf::decode(r, &mut tb)?;
    Ok(())
  }


  fn load_compiler_info(&mut self, r: &mut BinaryReader) -> Hopefully<()> {
    let mut tb = TermBuilder::new(&mut self.lit_heap);
    self.compiler_info = etf::decode(r, &mut tb)?;
    Ok(())
  }


  /// Approaching AtU8 section, populate atoms table in the Loader state.
  /// The format is: "Atom"|"AtU8", u32/big count { u8 length, "atomname" }.
  /// Formats are absolutely compatible except that Atom is latin-1
  fn load_atoms_utf8(&mut self, r: &mut BinaryReader) {
    let n_atoms = r.read_u32be();
    for _i in 0..n_atoms {
      let atom_bytes = r.read_u8();
      let atom_text = r.read_str_utf8(atom_bytes as Word).unwrap();
      self.raw.atoms.push(atom_text);
    }
  }


  /// Approaching Atom section, populate atoms table in the Loader state.
  /// The format is: "Atom"|"AtU8", u32/big count { u8 length, "atomname" }.
  /// Same as `load_atoms_utf8` but interprets strings per-character as latin-1
  fn load_atoms_latin1(&mut self, r: &mut BinaryReader) {
    let n_atoms = r.read_u32be();
    self.raw.atoms.reserve(n_atoms as usize);
    for _i in 0..n_atoms {
      let atom_bytes = r.read_u8();
      let atom_text = r.read_str_latin1(atom_bytes as Word).unwrap();
      self.raw.atoms.push(atom_text);
    }
  }


  fn set_mod_id(&mut self, code_server: &mut CodeServer) {
    assert!(!self.vm_atoms.is_empty());
    let mod_name = self.vm_atoms[0];
    let ver = code_server.next_module_version(mod_name);
    self.mod_id = Some(VersionedModuleId::new(mod_name, ver))
  }


  /// Load the `Code` section
  fn load_code(&mut self, r: &mut BinaryReader, chunk_sz: Word) {
    let _code_ver = r.read_u32be();
    let _min_opcode = r.read_u32be();
    let _max_opcode = r.read_u32be();
    let _n_labels = r.read_u32be();
    let _n_funs = r.read_u32be();
    // println!("Code section version {}, opcodes {}-{}, labels: {}, funs: {}",
    //  code_ver, min_opcode, max_opcode, n_labels, n_funs);

    self.raw.code = r.read_bytes(chunk_sz - 20).unwrap();
  }


  /// Read the imports table.
  /// Format is u32/big count { modindex: u32, funindex: u32, arity: u32 }
  fn load_imports(&mut self, r: &mut BinaryReader) {
    let n_imports = r.read_u32be();
    self.raw.imports.reserve(n_imports as usize);
    for _i in 0..n_imports {
      let imp = LImport {
        mod_atom_i: r.read_u32be(),
        fun_atom_i: r.read_u32be(),
        arity: r.read_u32be() as Arity,
      };
      self.raw.imports.push(imp);
    }
  }


  /// Read the exports or local functions table (same format).
  /// Format is u32/big count { funindex: u32, arity: u32, label: u32 }
  fn load_exports(&mut self, r: &mut BinaryReader) -> Vec<LExport> {
    let n_exports = r.read_u32be();
    let mut exports = Vec::new();
    exports.reserve(n_exports as usize);
    for _i in 0..n_exports {
      let exp = LExport {
        fun_atom_i: r.read_u32be(),
        arity: r.read_u32be() as Arity,
        label: r.read_u32be(),
      };
      exports.push(exp);
    }
    exports
  }


  fn load_fun_table(&mut self, r: &mut BinaryReader) {
    let n_funs = r.read_u32be();
    self.raw.lambdas.reserve(n_funs as usize);
    for _i in 0..n_funs {
      let fun_atom = r.read_u32be();
      let arity = r.read_u32be();
      let code_pos = r.read_u32be();
      let index = r.read_u32be();
      let nfree = r.read_u32be();
      let ouniq = r.read_u32be();
      self.raw.lambdas.push(LFun {
        fun_atom_i: fun_atom,
        arity: arity as Arity,
        code_pos,
        index,
        nfree,
        ouniq
      })
    }
  }


  fn load_line_info(&mut self, r: &mut BinaryReader) {
    let _version = r.read_u32be(); // must match emulator version 0
    let _flags = r.read_u32be();
    let _n_line_instr = r.read_u32be();
    let n_line_refs = r.read_u32be();
    let n_filenames = r.read_u32be();
    let mut _fname_index = 0u32;

    for _i in 0..n_line_refs {
      match compact_term::read(r).unwrap() {
        FTerm::SmallInt(_w) => {
          // self.linerefs.push((_fname_index, w));
        }
        FTerm::Atom(a) => _fname_index = a as u32,
        other => panic!("{}Unexpected data in line info section: {:?}",
                        module(), other)
      }
    }

    for _i in 0..n_filenames {
      let name_size = r.read_u16be();
      let _fstr = r.read_str_utf8(name_size as Word);
    }
  }


  /// Given the `r`, reader positioned on the contents of "LitT" chunk,
  /// decompress it and feed into `self.decode_literals/1`
  fn load_literals(&mut self, r: &mut BinaryReader, chunk_sz: Word) {
    // Red uncompressed size and reserve some memory
    let uncomp_sz = r.read_u32be();
    let mut inflated = Vec::<u8>::new();
    inflated.reserve(uncomp_sz as usize);

    // Deduce the 4 bytes uncomp_sz
    let deflated = r.read_bytes(chunk_sz - 4).unwrap();
    //dump_vec(&deflated);

    // Decompress deflated literal table
    let iocursor = Cursor::new(&deflated);
    zlib::Decoder::new(iocursor).read_to_end(&mut inflated).unwrap();
    assert_eq!(inflated.len(), uncomp_sz as usize,
               "{}LitT inflate failed", module());

    // Parse literal table
    //dump_vec(&inflated);
    self.decode_literals(inflated);
  }


  /// Given `inflated`, the byte contents of literal table, read the u32/big
  /// `count` and for every encoded term skip u32 and parse the external term
  /// format. Boxed values will go into the `self.lit_heap`.
  fn decode_literals(&mut self, inflated: Vec<u8>) {
    //dump_vec(&inflated);

    // Decode literals into literal heap here
    let mut r = BinaryReader::from_bytes(inflated);
    let count = r.read_u32be();
    self.lit_tab.reserve(count as usize);

    for _i in 0..count {
      // size should match actual consumed ETF bytes so can skip it here
      let _size = r.read_u32be();

      let mut tb = TermBuilder::new(&mut self.lit_heap);
      let literal = etf::decode(&mut r, &mut tb).unwrap();

      self.lit_tab.push(literal);
    }
  }


  /// Assume that loader raw structures are completed, and atoms are already
  /// transferred to the VM, we can now parse opcodes and their args.
  /// 'drained_code' is 'raw_code' moved out of 'self'
  fn postprocess_parse_raw_code(&mut self) -> Hopefully<()> {
    // Dirty swap to take raw_code out of self and give it to the binary reader
    let mut raw_code: Vec<u8> = Vec::new();
    mem::swap(&mut self.raw.code, &mut raw_code);

    //
    // Estimate code size and preallocate the code storage
    // TODO: This step is not efficient and does double parse of all args
    //
    let mut r = BinaryReader::from_bytes(raw_code);

    let code_size = {
      let mut s = 0usize;
      while !r.eof() {
        let op: opcode::RawOpcode = r.read_u8();
        let arity = gen_op::opcode_arity(op) as usize;
        for _i in 0..arity {
          let _arg0 = compact_term::read(&mut r).unwrap();
        }
        s += arity + 1;
      }
      s
    };
    self.code.reserve(code_size);

    let debug_code_start = self.code.as_ptr();
    //println!("Code_size {} code_start {:p}", code_size, debug_code_start);

    //
    // Writing code unpacked to words here. Break at every new function_info.
    //
    r.reset();
    while !r.eof() {
      // Read the opcode from the code section
      let op: opcode::RawOpcode = r.read_u8();

      // Read `arity` args, and convert them to reasonable runtime values
      let arity = gen_op::opcode_arity(op);
      let mut args: Vec<FTerm> = Vec::new();
      for _i in 0..arity {
        let arg0 = compact_term::read(&mut r).unwrap();
        // Atom_ args now can be converted to Atom (VM atoms)
        let arg1 = match self.resolve_loadtime_values(&arg0) {
          Some(tmp) => tmp,
          None => arg0
        };
        args.push(arg1);
      }

      match op {
        // add nothing for label, but record its location
        x if x == gen_op::OPCODE_LABEL => {
          if let FTerm::SmallInt(f) = args[0] {
            // Store weak ptr to function and code offset to this label
            let floc = self.code.len();
            self.labels.insert(LabelId(f as Word),
                               CodeOffset(floc));
          } else {
            op_badarg_panic(op, &args, 0);
          }
        }

        // add nothing for line, but TODO: Record line contents
        x if x == gen_op::OPCODE_LINE => {}

        // else push the op and convert all args to LTerms, also remember
        // code offsets for label values
        _ => {
          if op == gen_op::OPCODE_FUNC_INFO {
            // arg[0] mod name, arg[1] fun name, arg[2] arity
            let funarity = FunArity {
              f: args[1].to_lterm(&mut self.lit_heap),
              arity: args[2].loadtime_word() as Arity
            };

            // Function code begins after the func_info opcode (1+3)
            let fun_begin = self.code.len() + 4;
            match self.mod_id {
              Some(_mod_id) => {
                self.funs.insert(funarity, fun_begin);
                ()
              },
              None => panic!("{}mod_id must be set at this point", module()),
            }
          }

          self.code.push(opcode::to_memory_word(op));
          self.store_opcode_args(&args)?;
        } // case _
      } // match op
    } // while !r.eof

    assert_eq!(debug_code_start, self.code.as_ptr(),
               "{}Must do no reallocations", module());
    Ok(())
  }


  /// Given arity amount of `args` from another opcode, process them and store
  /// into the `self.code` array. `LoadTimeExtList` get special treatment as a
  /// container of terms. `LoadTimeLabel` get special treatment as we try to
  /// resolve them into an offset.
  fn store_opcode_args(&mut self, args: &[FTerm]) -> Hopefully<()> {
    for a in args {
      match *a {
        // Ext list is special so we convert it and its contents to lterm
        FTerm::LoadTimeExtlist(ref jtab) => {
          // Push a header word with length
          let heap_jtab = boxed::Tuple::create_into(
            &mut self.lit_heap, jtab.len()
          )?;
          self.code.push(heap_jtab.make_term().raw());

          // Each value convert to LTerm and also push forming a tuple
          for (index, t) in jtab.iter().enumerate() {
            let new_t = if let FTerm::LoadTimeLabel(f) = *t {
              // Try to resolve labels and convert now, or postpone
              let ploc = PatchLocation::PatchJtabElement(heap_jtab.make_term(), index);
              self.maybe_convert_label(LabelId(f), ploc)
            } else {
              t.to_lterm(&mut self.lit_heap).raw()
            };

            unsafe { heap_jtab.set_raw_word_base0(index, new_t) }
          }
        }

        // Label value is special, we want to remember where it was
        // to convert it to an offset
        FTerm::LoadTimeLabel(f) => {
          let ploc = PatchLocation::PatchCodeOffset(self.code.len());
          let new_t = self.maybe_convert_label(LabelId(f),
                                               ploc);
          self.code.push(new_t)
        }

        // Load-time literals are already loaded on `self.lit_heap`
        FTerm::LoadTimeLit(lit_index) => {
          self.code.push(self.lit_tab[lit_index].raw())
        }

          // Otherwise convert via a simple method
        _ => self.code.push(a.to_lterm(&mut self.lit_heap).raw()),
      }
    } // for a in args
    Ok(())
  }


  /// Given label index `l` check if it is known, then return a new jump
  /// destination - a boxed code location pointer to be used by the caller.
  /// Otherwise the `patch_location` is stored to `self.replace_labels` to be
  /// processed later and a `SmallInt` is returned to be used temporarily.
  fn maybe_convert_label(&mut self, l: LabelId,
                         patch_loc: PatchLocation) -> Word {
    // Resolve the label, if exists in labels table
    match self.labels.get(&l) {
      Some(offset0) =>
        self.create_jump_destination(*offset0),
      None => {
        self.replace_labels.push(patch_loc);
        let LabelId(label_id) = l;
        LTerm::make_small_unsigned(label_id).raw()
      }
    }
  }


  /// Given label destination and `self.code` length calculate a relative
  /// signed jump offset for it.
  fn create_jump_destination(&self, dst_offset: CodeOffset) -> Word {
    let CodeOffset(offs) = dst_offset;
    let ptr = &self.code[offs] as *const Word;
    LTerm::make_cp(ptr).raw()
  }


  /// Analyze the code and replace label values with known label locations.
  fn postprocess_fix_labels(&mut self) {
    // Postprocess self.replace_labels, assuming that at this point labels exist
    let mut repl = Vec::<PatchLocation>::new();
    mem::swap(&mut repl, &mut self.replace_labels);

    for ploc in &repl {
      match *ploc {
        PatchLocation::PatchCodeOffset(cmd_offset) => {
          let val = LTerm::from_raw(self.code[cmd_offset]);
          self.code[cmd_offset] = self.postprocess_fix_1_label(val)
        },

        PatchLocation::PatchJtabElement(jtab, index) => {
          let jtab_ptr = boxed::Tuple::from_ptr(jtab.box_ptr_mut());
          unsafe {
            let val = jtab_ptr.get_element_base0(index);
            jtab_ptr.set_raw_word_base0(index,
                                       self.postprocess_fix_1_label(val))
          }
        }
      } // match
    } // for ploc
  }


  /// Helper for `postprocess_fix_label`, takes a word from code memory or from
  /// a jump table, resolves as if it was a label index, and returns a value
  /// to be put back into memory.
  fn postprocess_fix_1_label(&self, val: LTerm) -> Word {
    // Convert from LTerm smallint to integer and then to labelid
    let unfixed = val.small_get_s() as Word;

    // Zero label id means no location, so we will store NIL [] there
    if unfixed > 0 {
      let unfixed_l = LabelId(unfixed);

      // Lookup the label. Crash here if bad label.
      let dst_offset = self.labels[&unfixed_l];

      // Update code cell with special label value
      self.create_jump_destination(dst_offset)
    } else {
      // Update code cell with no-value
      LTerm::nil().raw()
    }
  }


  /// Analyze the code and for certain opcodes overwrite their import index
  /// args with direct pointer to import heap.
  fn postprocess_setup_imports(&mut self) -> Hopefully<()> {
    // Step 1
    // Write imports onto literal heap as {Mod, Fun, Arity} triplets
    //
    self.imports.reserve(self.raw.imports.len());
    for ri in &self.raw.imports {
      let mod_atom = self.atom_from_loadtime_index(ri.mod_atom_i);
      let fun_atom = self.atom_from_loadtime_index(ri.fun_atom_i);
      let mf_arity = MFArity::new(mod_atom, fun_atom, ri.arity);
      let is_bif = bif::is_bif(&mf_arity);
      //println!("is_bif {} for {}", is_bif, mf_arity);
      let ho_imp = unsafe {
        boxed::Import::place_into(&mut self.lit_heap, mf_arity, is_bif)?
      };

      self.imports.push(ho_imp);
    }

    // Step 2
    // For each opcode if it has import index arg - overwrite it
    //
    let c_iter = unsafe {
      code::iter::create_mut(&mut self.code)
    };
    for cp in c_iter {
      let curr_opcode = opcode::from_memory_ptr(cp.ptr());
      match curr_opcode {
        gen_op::OPCODE_MAKE_FUN2 => {
          // arg[0] is export
//          self.rewrite_import_index_arg(&cp, 1)
          self.rewrite_lambda_index_arg(cp, 1)
        },
        gen_op::OPCODE_BIF1 |
        gen_op::OPCODE_BIF2 |
        gen_op::OPCODE_CALL_EXT |
        gen_op::OPCODE_CALL_EXT_LAST |
        gen_op::OPCODE_CALL_EXT_ONLY => {
          // arg[1] is export
          self.rewrite_import_index_arg(cp, 2)
        },
        gen_op::OPCODE_GC_BIF1 |
        gen_op::OPCODE_GC_BIF2 |
        gen_op::OPCODE_GC_BIF3 => {
          // arg[2] is export
          self.rewrite_import_index_arg(cp, 3)
        }
        _ => {}
      }
    }
    Ok(())
  }


  /// Internal helper which takes N'th arg of an opcode, parses it as a small
  /// unsigned and writes an LTerm pointer to a literal {M,F,Arity} tuple.
  fn rewrite_import_index_arg(&self, cp: CodePtrMut, n: isize) {
    let import0 = unsafe { LTerm::from_raw(cp.read_n(n)) };
    let import1 = self.imports[import0.small_get_u()].raw();
    unsafe { cp.write_n(n, import1) }
  }


  /// Given a pointer to a `make_fun2` or similar opcode with a lambda index
  /// argument, replace it with a raw pointer to a loaded `FunEntry`.
  /// The `FunEntry` will be owned by the module we're loading, and will be
  /// freed together with the code, so it should be safe to use the pointer.
  fn rewrite_lambda_index_arg(&self, cp: CodePtrMut, n: isize) {
    let lambda_i = unsafe { LTerm::from_raw(cp.read_n(n)) };
    let lambda_p = &self.lambdas[lambda_i.small_get_u()] as *const FunEntry;
    let lambda_term = LTerm::make_cp(lambda_p as *const Word);
    unsafe { cp.write_n(n, lambda_term.raw()) }
  }


  /// Given a load-time `Atom_` or a structure possibly containing `Atom_`s,
  /// resolve it to a runtime atom index using a lookup table.
  pub fn resolve_loadtime_values(&self, arg: &FTerm) -> Option<FTerm> {
    match *arg {
      // A special value 0 means NIL []
      FTerm::LoadTimeAtom(0) => Some(FTerm::Nil),

      // Repack load-time atom via an `LTerm` index into an `FTerm` atom
      FTerm::LoadTimeAtom(i) => {
        let aindex = self.atom_from_loadtime_index(i).atom_index();
        Some(FTerm::Atom(aindex))
      },

      // ExtList_ can contain Atom_ - convert them to runtime Atoms
      FTerm::LoadTimeExtlist(ref lst) => {
        let mut result: Vec<FTerm> = Vec::new();
        result.reserve(lst.len());
        for x in lst.iter() {
          match self.resolve_loadtime_values(x) {
            Some(tmp) => result.push(tmp),
            None => result.push(x.clone())
          }
        };
        Some(FTerm::LoadTimeExtlist(result))
      },
      // Otherwise no changes
      _ => None
    }
  }
}

/// Report a bad opcode arg
// TODO: Use this more, than just label opcode
fn op_badarg_panic(op: u8, args: &[FTerm], argi: Word) {
  panic!("{}Opcode {} the arg #{} in {:?} is bad", module(), op, argi, args)
}
