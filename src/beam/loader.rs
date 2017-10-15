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

use beam::compact_term;
use beam::gen_op;
use defs::{Word, Arity};
use emulator::atom;
use emulator::code::{LabelId, CodeOffset, Code, opcode};
use emulator::disasm;
use emulator::funarity::FunArity;
use emulator::heap::{Heap, DEFAULT_LIT_HEAP};
use emulator::module;
use fail::{Hopefully, Error};
use term::fterm::FTerm;
use term::lterm::LTerm;
use util::bin_reader::BinaryReader;
use util::ext_term_format as etf;
//use util::print::dump_vec;


pub fn module() -> &'static str { "beam::loader: " }


/// Imports table item (mfa's referred by this module).
/// Raw data structure as loaded from BEAM file
#[allow(dead_code)]
struct LImport {
  mod_atom: u32,
  fun_atom: u32,
  arity: Arity,
}


/// Exports table item, as specified in `-export()` attribute.
/// Raw data structure as loaded from BEAM file.
#[allow(dead_code)]
struct LExport {
  fun_atom: u32,
  arity: Arity,
  label: u32,
}


/// Function closures used in this file, with info on captured values.
/// Raw data structure as loaded from BEAM file
#[allow(dead_code)]
struct LFun {
  fun_atom: u32,
  arity: u32,
  code_pos: u32,
  index: u32,
  nfree: u32,
  ouniq: u32,
}


/// BEAM loader state.
pub struct Loader {
  //--- Stage 1 raw structures ---
  /// Raw atoms loaded from BEAM module as strings
  raw_atoms: Vec<String>,
  raw_imports: Vec<LImport>,
  raw_exports: Vec<LExport>,
  raw_locals: Vec<LExport>,
  raw_funs: Vec<LFun>,
  /// Temporary storage for loaded code, will be parsed in stage 2
  raw_code: Vec<u8>,

  //--- Stage 2 structures filled later ---
  /// Atoms converted to VM terms
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
  replace_labels: Vec<CodeOffset>,
  funs: module::FunTable,
  /// Literal table decoded into friendly terms (does not use process heap).
  lit_tab: Vec<LTerm>,
  /// A place to allocate larger lterms (literal heap)
  lit_heap: Heap,
  /// Proplist of module attributes as loaded from "Attr" section
  mod_attrs: LTerm,
  /// Compiler flags as loaded from "Attr" section
  compiler_info: LTerm,
}


impl Loader {
  /// Construct a new loader state.
  pub fn new() -> Loader {
    Loader {
      raw_atoms: Vec::new(),
      raw_imports: Vec::new(),
      raw_exports: Vec::new(),
      raw_locals: Vec::new(),
      raw_funs: Vec::new(),
      raw_code: Vec::new(),

      lit_tab: Vec::new(),
      lit_heap: Heap::new(DEFAULT_LIT_HEAP),
      vm_atoms: Vec::new(),
      //vm_funs: BTreeMap::new(),

      code: Vec::new(),
      labels: BTreeMap::new(),
      replace_labels: Vec::new(),
      funs: BTreeMap::new(),
      mod_attrs: LTerm::nil(),
      compiler_info: LTerm::nil(),
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
        Err(Error::CodeLoadingPrematureEOF) => break,
        Err(e) => return Err(e)
      };
      let chunk_sz = r.read_u32be();
      let pos_begin = r.pos();

      //println!("Chunk {}", chunk_h);
      match chunk_h.as_ref() {
        "Atom" => self.load_atoms_latin1(&mut r),
        "Attr" => self.load_attributes(&mut r),
        "AtU8" => self.load_atoms_utf8(&mut r),
        "Code" => self.load_code(&mut r, chunk_sz as Word),
        "ExpT" => self.raw_exports = self.load_exports(&mut r),
        "FunT" => self.load_fun_table(&mut r),
        "ImpT" => self.load_imports(&mut r),
        "Line" => self.load_line_info(&mut r),
        "LocT" => self.raw_locals = self.load_exports(&mut r),
        "LitT" => self.load_literals(&mut r, chunk_sz as Word),

        "CInf" | // skip compiler info
        "Dbgi" | // skip debug info
        "StrT" | // skip strings TODO load strings?
        "Abst" => r.skip(chunk_sz as Word), // skip abstract code

        other => {
          let msg = format!("{}Unexpected chunk: {}", module(), other);
          return Err(Error::CodeLoadingFailed(msg));
        }
      }

      // The next chunk is aligned at 4 bytes
      let aligned_sz = 4 * ((chunk_sz + 3) / 4);
      r.seek(pos_begin + aligned_sz as Word);
    }

    Ok(())
  }


  /// Call this to apply changes to the VM after module loading succeeded. The
  /// module object is not created yet, but some effects like atoms table
  /// we can already apply.
  pub fn load_stage2(&mut self) {
    self.vm_atoms.reserve(self.raw_atoms.len());
    for a in &self.raw_atoms {
      self.vm_atoms.push(atom::from_str(a));
    }

    self.postprocess_code_section();
    self.fix_labels();
  }


  /// At this point loading is finished, and we create Erlang module and
  /// return a reference counted pointer to it. VM (the caller) is responsible
  /// for adding the module to its code registry.
  pub fn load_finalize(&mut self) -> Hopefully<module::Ptr> {
    let mod_name = self.vm_atoms[0];
    let newmod = module::Module::new(mod_name);

    // Move funs into new module
    {
      let mut mod1 = newmod.borrow_mut();
      mem::swap(&mut self.funs, &mut mod1.funs);
      mem::swap(&mut self.code, &mut mod1.code);
      mem::swap(&mut self.lit_heap, &mut mod1.lit_heap);

      unsafe {
        disasm::disasm(&mod1.code, None);
        //mod1.lit_heap.dump()
      };
    }

    Ok(newmod)
  }

  //============================================================================

  /// Read Attr section: two terms (module attributes and compiler info) encoded
  /// as external term format.
  fn load_attributes(&mut self, r: &mut BinaryReader) {
    self.mod_attrs = etf::decode(r, &mut self.lit_heap).unwrap();
    self.compiler_info = etf::decode_naked(r, &mut self.lit_heap).unwrap();
  }


  /// Approaching AtU8 section, populate atoms table in the Loader state.
  /// The format is: "Atom"|"AtU8", u32/big count { u8 length, "atomname" }.
  /// Formats are absolutely compatible except that Atom is latin-1
  fn load_atoms_utf8(&mut self, r: &mut BinaryReader) {
    let n_atoms = r.read_u32be();
    for _i in 0..n_atoms {
      let atom_bytes = r.read_u8();
      let atom_text = r.read_str_utf8(atom_bytes as Word).unwrap();
      self.raw_atoms.push(atom_text);
    }
  }


  /// Approaching Atom section, populate atoms table in the Loader state.
  /// The format is: "Atom"|"AtU8", u32/big count { u8 length, "atomname" }.
  /// Same as `load_atoms_utf8` but interprets strings per-character as latin-1
  fn load_atoms_latin1(&mut self, r: &mut BinaryReader) {
    let n_atoms = r.read_u32be();
    self.raw_atoms.reserve(n_atoms as usize);
    for _i in 0..n_atoms {
      let atom_bytes = r.read_u8();
      let atom_text = r.read_str_latin1(atom_bytes as Word).unwrap();
      self.raw_atoms.push(atom_text);
    }
  }


  /// Load the `Code` section
  fn load_code(&mut self, r: &mut BinaryReader, chunk_sz: Word) {
    let _code_ver = r.read_u32be();
    let _min_opcode = r.read_u32be();
    let _max_opcode = r.read_u32be();
    let _n_labels = r.read_u32be();
    let _n_funs = r.read_u32be();
    //    println!("Code section version {}, opcodes {}-{}, labels: {}, funs: {}",
    //      code_ver, min_opcode, max_opcode, n_labels, n_funs);

    self.raw_code = r.read_bytes(chunk_sz - 20).unwrap();
  }


  /// Read the imports table.
  /// Format is u32/big count { modindex: u32, funindex: u32, arity: u32 }
  fn load_imports(&mut self, r: &mut BinaryReader) {
    let n_imports = r.read_u32be();
    self.raw_imports.reserve(n_imports as usize);
    for _i in 0..n_imports {
      let imp = LImport {
        mod_atom: r.read_u32be(),
        fun_atom: r.read_u32be(),
        arity: r.read_u32be() as Arity,
      };
      self.raw_imports.push(imp);
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
        fun_atom: r.read_u32be(),
        arity: r.read_u32be() as Arity,
        label: r.read_u32be(),
      };
      exports.push(exp);
    }
    exports
  }


  fn load_fun_table(&mut self, r: &mut BinaryReader) {
    let n_funs = r.read_u32be();
    self.raw_funs.reserve(n_funs as usize);
    for _i in 0..n_funs {
      let fun_atom = r.read_u32be();
      let arity = r.read_u32be();
      let code_pos = r.read_u32be();
      let index = r.read_u32be();
      let nfree = r.read_u32be();
      let ouniq = r.read_u32be();
      self.raw_funs.push(LFun {
        fun_atom,
        arity,
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


  /// Give the `r`, reader positioned on the contents of "LitT" chunk,
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
    assert_eq!(inflated.len(), uncomp_sz as usize, "LitT inflate failed");

    // Parse literal table
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
      let lterm = etf::decode(&mut r, &mut self.lit_heap).unwrap();
      self.lit_tab.push(lterm);
    }
  }


    /// Assume that loader raw structures are completed, and atoms are already
  /// transferred to the VM, we can now parse opcodes and their args.
  /// 'drained_code' is 'raw_code' moved out of 'self'
  fn postprocess_code_section(&mut self) {
    // Dirty swap to take raw_code out of self and give it to the binary reader
    let mut raw_code: Vec<u8> = Vec::new();
    mem::swap(&mut self.raw_code, &mut raw_code);
    let mut r = BinaryReader::from_bytes(raw_code);

    // Writing code unpacked to words here. Break at every new function_info.
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
          if let FTerm::LoadTimeInt(f) = args[0] {
            // Store weak ptr to function and code offset to this label
            let floc = self.code.len();
            self.labels.insert(LabelId::Val(f), CodeOffset::Val(floc));
          } else {
            op_badarg_panic(op, &args, 0);
          }
        }

        // add nothing for line, but TODO: Record line contents
        x if x == gen_op::OPCODE_LINE => {}

        x if x == gen_op::OPCODE_FUNC_INFO => {
          // arg[0] mod name, arg[1] fun name, arg[2] arity
          let funarity = FunArity {
            f: args[1].to_lterm(&mut self.lit_heap),
            arity: args[2].loadtime_word() as Arity
          };
          self.funs.insert(funarity, CodeOffset::Val(self.code.len()));
        }

        // else push the op and convert all args to LTerms, also remember
        // code offsets for label values
        _ => {
          self.code.push(opcode::to_memory_word(op));
          self.postprocess_store_args(&args);
        } // case _
      } // match op
    } // while !r.eof
  }


  /// Given arity amount of `args` from another opcode, process them and store
  /// into the `self.code` array. `LoadTimeExtList` get special treatment as a
  /// container of terms. `LoadTimeLabel` get special treatment as we try to
  /// resolve them into an offset.
  fn postprocess_store_args(&mut self, args: &[FTerm]) {
    for a in args {
      match *a {
        // Ext list is special so we convert it and its contents to lterm
        FTerm::LoadTimeExtlist(ref jtab) => {
          // Push a header word with length
          let heap_jtab = self.lit_heap.allocate_tuple(jtab.len()).unwrap();
          self.code.push(heap_jtab.make_tuple().raw());

          // Each value convert to LTerm and also push forming a tuple
          let mut index = 0usize;
          for t in jtab.iter() {
            let new_t = if let FTerm::LoadTimeLabel(f) = *t {
              // Try to resolve labels and convert now, or postpone
              self.maybe_convert_label(LabelId::Val(f))
            } else {
              t.to_lterm(&mut self.lit_heap).raw()
            };

            unsafe { heap_jtab.set_raw_word_base0(index, new_t) }
            index += 1;
          }
        }

        // Label value is special, we want to remember where it was
        // to convert it to an offset
        FTerm::LoadTimeLabel(f) => {
          let new_t = self.maybe_convert_label(LabelId::Val(f));
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
  }


  /// Given label index `l` check if it is known, then return a new `Label`
  /// LTerm to be pushed into the code by the caller. Otherwise push its code
  /// location to `self.replace_labels` to be processed later and store a
  /// `SmallInt` LTerm in the code temporarily.
  fn maybe_convert_label(&mut self, l: LabelId) -> Word {
    // Resolve the label, if exists in labels table
    match self.labels.get(&l) {
      Some(offset0) =>
        self.create_jump_destination(offset0),
      None => {
        self.replace_labels.push(CodeOffset::Val(self.code.len()));
        let LabelId::Val(label_id) = l;
        LTerm::make_small_u(label_id).raw()
      }
    }
  }


  /// Given label destination and `self.code` length calculate a relative
  /// signed jump offset for it.
  fn create_jump_destination(&self, dst_offset: &CodeOffset) -> Word {
    let &CodeOffset::Val(offs) = dst_offset;
    let ptr = &self.code[offs] as *const Word;
    LTerm::make_box(ptr).raw()
  }


  /// Analyze the code and replace label values with known label locations.
  fn fix_labels(&mut self) {
    // Postprocess self.replace_labels, assuming that at this point labels exist
    let mut repl = Vec::<CodeOffset>::new();
    mem::swap(&mut repl, &mut self.replace_labels);
    for code_offs in &repl {
      // Read code cell
      let &CodeOffset::Val(cmd_offset) = code_offs;

      // Convert from LTerm smallint to integer and then to labelid
      let unfixed = LTerm::from_raw(self.code[cmd_offset]);
      let unfixed_l = LabelId::Val(unfixed.small_get_s() as Word);

      // Lookup the label. Crash here if bad label.
      let dst_offset = &self.labels[&unfixed_l];

      // Update code cell with special label value
      self.code[cmd_offset] = self.create_jump_destination(dst_offset);
    }
  }


/// Given a load-time `Atom_` or a structure possibly containing `Atom_`s,
/// resolve it to a runtime atom index using a lookup table.
  pub fn resolve_loadtime_values(&self, arg: &FTerm) -> Option<FTerm> {
    match *arg {
      // A special value 0 means NIL []
      FTerm::LoadTimeAtom(0) => Some(FTerm::Nil),

      // Repack load-time atom into a runtime atom
      FTerm::LoadTimeAtom(i) => {
        let aindex = self.vm_atoms[i-1].atom_index();
        Some(FTerm::Atom(aindex))
      },

      //FTerm::LoadTimeLit(_) => None, // do not convert yet

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

/// Report a bad opcode arg. TODO: Use this more, than just label opcode
fn op_badarg_panic(op: u8, args: &[FTerm], argi: Word) {
  panic!("{}Opcode {} the arg #{} in {:?} is bad", module(), op, argi, args)
}
