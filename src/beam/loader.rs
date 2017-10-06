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

use beam::compact_term;
use beam::gen_op;
use defs::{Word, Arity};
use emulator::funarity::FunArity;
use emulator::function;
use emulator::module;
use emulator::vm::VM;
use fail::{Hopefully, Error};
use term::fterm::FTerm;
use term::lterm::LTerm;
use util::bin_reader::BinaryReader;


pub fn module() -> &'static str { "beam::loader: " }


/// Raw data structure as loaded from BEAM file
struct LImport {
  mod_atom: u32,
  fun_atom: u32,
  arity: Arity,
}

/// Raw data structure as loaded from BEAM file
struct LExport {
  fun_atom: u32,
  arity: Arity,
  label: u32,
}

/// Raw data structure as loaded from BEAM file
struct LFun {
  fun_atom: u32,
  arity: u32,
  code_pos: u32,
  index: u32,
  nfree: u32,
  ouniq: u32,
}

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
  vm_funs: BTreeMap<FunArity, function::Ptr>,
  /// Labels are stored here while loading, for later resolve
  labels: BTreeMap<Word, module::CodeLabel>,
  /// For postprocessing: Current function/arity from func_info opcode
  funarity: FunArity,
  /// Locations of label values are collected and at a later pass replaced
  /// with their word values or function pointer (if the label points outside)
  replace_labels: Vec<Word>,
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

      vm_atoms: Vec::new(),
      vm_funs: BTreeMap::new(),
      labels: BTreeMap::new(),
      funarity: FunArity::new_uninit(),
      replace_labels: Vec::new(),
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

      //      println!("Chunk {}", chunk_h);
      match chunk_h.as_ref() {
        "Atom" => self.load_atoms_latin1(&mut r),
        "Attr" => r.skip(chunk_sz as Word), // TODO: read attributes
        "AtU8" => self.load_atoms_utf8(&mut r),
        "CInf" => r.skip(chunk_sz as Word),
        "Code" => self.load_code(&mut r, chunk_sz as Word),
        "Dbgi" => r.skip(chunk_sz as Word),
        "ExpT" => self.raw_exports = self.load_exports(&mut r),
        "FunT" => self.load_fun_table(&mut r),
        "ImpT" => self.load_imports(&mut r),
        "Line" => self.load_line_info(&mut r),
        "LocT" => self.raw_locals = self.load_exports(&mut r),
        "StrT" => r.skip(chunk_sz as Word),
        other => {
          let msg = format!("{}Unexpected chunk: {}", module(), other);
          return Err(Error::CodeLoadingFailed(msg));
        }
      }

      // The next chunk is aligned at 4 bytes
      let aligned_sz = 4 * ((chunk_sz + 3) / 4);
      let align = aligned_sz - chunk_sz;
      if align > 0 { r.skip(align as Word); }
    }

    Ok(())
  }


  /// Call this to apply changes to the VM after module loading succeeded. The
  /// module object is not created yet, but some effects like atoms table
  /// we can already apply.
  pub fn load_stage2(&mut self, vm: &mut VM) {
    self.vm_atoms.reserve(self.raw_atoms.len());
    for a in &self.raw_atoms {
      self.vm_atoms.push(vm.atom(&a));
    }

    self.postprocess_code_section();
  }


  /// At this point loading is finished, and we create Erlang module and
  /// return a reference counted pointer to it. VM (the caller) is responsible
  /// for adding the module to its code registry.
  pub fn load_finalize(&mut self) -> Hopefully<module::Ptr> {
    let mod_name = self.vm_atoms[0];
    let newmod = module::Module::new(mod_name);

    //self.print_funs();

    // Move funs into new module
    {
      let mut mod1 = newmod.borrow_mut();
      mem::swap(&mut self.vm_funs, &mut mod1.funs);
      mem::swap(&mut self.labels, &mut mod1.labels);
    }

    Ok(newmod)
  }

  //============================================================================

  // Print disassembly of loaded functions
  #[cfg(feature = "dev_build")]
  fn print_funs(&self) {
    for (_k, f) in self.vm_funs.iter() {
      let fun = f.borrow();
      println!("------ Function {} ------", fun.funarity);
      fun.disasm();
    }
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


  /// Assume that loader raw structures are completed, and atoms are already
  /// transferred to the VM, we can now parse opcodes and their args.
  /// 'drained_code' is 'raw_code' moved out of 'self'
  fn postprocess_code_section(&mut self) {
    // Dirty swap to take raw_code out of self and give it to the binary reader
    let mut raw_code: Vec<u8> = Vec::new();
    mem::swap(&mut self.raw_code, &mut raw_code);
    let mut r = BinaryReader::from_bytes(raw_code);

    // Writing code unpacked to words here. Break at every new function_info.
    let mut fun_p = function::Function::new(self.vm_atoms[0].clone());

    while !r.eof() {
      // Read the u8 opcode
      let op = r.read_u8();

      // Read `arity` args, and convert them to reasonable runtime values
      let arity = gen_op::opcode_arity(op);
      let mut args: Vec<FTerm> = Vec::new();
      for _i in 0..arity {
        let arg0 = compact_term::read(&mut r).unwrap();
        // Atom_ args now can be converted to Atom (VM atoms)
        let arg1 = match arg0.maybe_resolve_atom_(&self.vm_atoms) {
          Some(tmp) => tmp,
          None => arg0
        };
        args.push(arg1);
      }

      match op {
        // add nothing for label, but record its location
        x if x == gen_op::OPCODE::Label as u8 => {
          if let FTerm::Int_(f) = args[0] {
            // Store weak ptr to function and code offset to this label
            let floc = module::CodeLabel {
              fun: function::make_weak(&fun_p),
              offset: fun_p.borrow().code.len(),
            };
            self.labels.insert(f, floc);
          } else { panic_postprocess_instr(op, &args, 0); }
        }

        // add nothing for line, but TODO: Record line contents
        x if x == gen_op::OPCODE::Line as u8 => {}

        x if x == gen_op::OPCODE::FuncInfo as u8 => {
          // arg[0] mod name, arg[1] fun name, arg[2] arity
          self.funarity = FunArity {
            f: args[1].to_lterm(),
            arity: args[2].loadtime_word() as Arity
          };
          // function finished, take it and start a new one
          self.commit_fun(fun_p.clone());
          fun_p = function::Function::new(self.vm_atoms[0].clone());
        }

        // else push the op and convert all args to LTerms, also remember
        // code offsets for label values
        _ => {
          function::with_fun_mut(&fun_p, &mut |f1: &mut function::Function| {
            f1.code.push(op as Word)
          });

          self.postprocess_store_args(&args, &fun_p);
        } // case _
      } // match op
    } // while !r.eof

    // final function finished also take it
    self.commit_fun(fun_p);
  }


  fn postprocess_store_args(&mut self, args: &Vec<FTerm>, fun_p: &function::Ptr) {
    for a in args {
      match a {
        // Ext list is special so we convert it and its contents to lterm
        &FTerm::ExtList_(ref jtab) => {
          function::with_fun_mut(&fun_p, &mut |f1: &mut function::Function| {
            // Push a header word with length
            f1.code.push(LTerm::make_header(jtab.len()).raw())
          });

          // Each value convert to LTerm and also push forming a tuple
          for t in jtab.iter() {
            let new_t = if let &FTerm::Label_(f) = t {
              // Try to resolve labels and convert now, or postpone
              self.push_term_or_convert_label(f, &fun_p)
            } else {
              t.to_lterm().raw()
            };
            function::with_fun_mut(&fun_p, &mut |f1: &mut function::Function| {
              f1.code.push(new_t);
            })
          }
        }
        // Label value is special, we want to remember where it was
        // to convert it to an offset
        &FTerm::Label_(f) => {
          let new_t = self.push_term_or_convert_label(f, &fun_p);
          function::with_fun_mut(&fun_p, &mut |f1: &mut function::Function| {
            f1.code.push(new_t)
          })
        }
        // Otherwise convert via a simple method
        _ => {
          function::with_fun_mut(&fun_p, &mut |f1: &mut function::Function| {
            f1.code.push(a.to_lterm().raw())
          })
        }
      }
    } // for a in args
  }


  /// Given label index f check if it is known, then return Label_ LTerm to
  /// be pushed into the code by the caller. Otherwise push code location to
  /// `replace_labels` and store an int in the code temporarily.
  fn push_term_or_convert_label(&mut self, label_id: Word,
                                fun_p: &function::Ptr) -> Word {
    // Resolve the label, if exists in labels table
    match self.labels.get(&label_id) {
      Some(code_label) => {
        let same_fun = {
          // Dig into the weak reference and get a readonly borrow
          let cl_fn0 = code_label.fun.upgrade().unwrap();
          let cl_fn = cl_fn0.borrow();

          // Local label - can convert immediately without postponing
          *cl_fn == *fun_p.borrow()
        };

        // Only do this if the function names are same
        if same_fun {
          return LTerm::make_label(code_label.offset).raw();
        }
      }
      None => {}
    };

    self.replace_labels.push(fun_p.borrow().code.len());
    LTerm::make_small_u(label_id).raw()
  }


  /// The function `fun` is almost ready, finalize labels resolution for those
  /// labels which weren't known in the load-time, but must be all known now,
  /// and then store it.
  fn commit_fun(&mut self, fun: function::Ptr) {
    self.commit_fix_labels(&mut fun.borrow_mut());
    self.commit_store_fun(fun);
  }


  /// Analyze the code and replace label values with known label locations.
  /// Some labels point to other functions within the module - replace them
  /// (cross-function labels) with function lookup results.
  fn commit_fix_labels(&mut self, fun: &mut function::Function) {
    // Postprocess self.replace_labels, assuming that at this point labels exist
    println!("Replace labels {:?}", self.replace_labels);
    let mut repl = Vec::<Word>::new();
    mem::swap(&mut repl, &mut self.replace_labels);
    for lloc in repl.iter() {
      let label_lterm = fun.code[*lloc];
    }
  }


  /// Store the fun, which has completed loading, into the module dictionary.
  fn commit_store_fun(&mut self, fun: function::Ptr) {
    fun.borrow().disasm();

    // Function is done, let's store it
    fun.borrow_mut().funarity = self.funarity.clone();
    self.vm_funs.insert(self.funarity.clone(), fun);
  }
} // impl


fn panic_postprocess_instr(op: u8, args: &Vec<FTerm>, argi: Word) {
  panic!("{}Opcode {} the arg #{} in {:?} is bad", module(), op, argi, args)
}
