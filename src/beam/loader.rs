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
use emulator::function;
use emulator::module;
use emulator::vm::VM;
use emulator::gen_op;
use rterror;
use emulator::funarity::FunArity;
use term::friendly::FTerm;
use term::low_level::LTerm;
use defs::{Word, Integral, Arity};
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

struct LLabel {
  fun: function::Weak,
  offset: Word,
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

  /// Labels are stored here while loading, for later resolve
  labels: BTreeMap<Word, LLabel>,
  /// For postprocessing: Current function/arity from func_info opcode
  funarity: FunArity,

  //--- Stage 2 structures filled later ---
  /// Atoms converted to VM terms
  vm_atoms: Vec<LTerm>,
  vm_funs: BTreeMap<(LTerm, Arity), function::Ptr>,
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

      labels: BTreeMap::new(),
      funarity: FunArity::new(),

      vm_atoms: Vec::new(),
      vm_funs: BTreeMap::new(),
    }
  }

  /// Loading the module. Validate the header and iterate over sections,
  /// then call `load_stage2()` to apply changes to the VM, and then finalize
  /// it by calling `load_finalize()` which will return you a module object.
  pub fn load(&mut self, fname: &PathBuf) -> Result<(), rterror::Error>
  {
    // Prebuffered BEAM file should be released as soon as the initial phase
    // is done. TODO: [Performance] Use memmapped file?
    let mut r = BinaryReader::from_file(fname);

    // Parse header and check file FOR1 signature
    let hdr1 = Bytes::from(&b"FOR1"[..]);
    r.ensure_bytes(&hdr1)?;

    let beam_sz = r.read_u32be();

    // Check BEAM signature
    let hdr2 = Bytes::from(&b"BEAM"[..]);
    r.ensure_bytes(&hdr2)?;

    while true {
      // EOF may strike here when we finished reading
      let chunk_h = match r.read_str_latin1(4) {
        Ok(s) => s,
        // EOF is not an error
        Err(rterror::Error::CodeLoadingPrematureEOF) => break,
        Err(e) => return Err(e)
      };
      let chunk_sz = r.read_u32be();

      println!("Chunk {}", chunk_h);
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
          return Err(rterror::Error::CodeLoadingFailed(msg))
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

    self.postprocess_code_section()
  }

  /// At this point loading is finished, and we create Erlang module and
  /// return a reference counted pointer to it. VM (the caller) is responsible
  /// for adding the module to its code registry.
  pub fn load_finalize(&mut self) -> Result<module::Ptr, rterror::Error> {
    let mod_name = self.vm_atoms[0];
    let newmod = module::Module::new(mod_name);
    for (_k, f) in self.vm_funs.iter() {
      let fun = f.borrow();
      println!("------ Function {} ------", fun.funarity);
      fun.disasm();
    }
    Ok(newmod)
  }

  //============================================================================

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
    for i in 0..n_atoms {
      let atom_bytes = r.read_u8();
      let atom_text = r.read_str_latin1(atom_bytes as Word).unwrap();
      self.raw_atoms.push(atom_text);
    }
  }

  /// Load the `Code` section
  fn load_code(&mut self, r: &mut BinaryReader, chunk_sz: Word) {
    let code_ver = r.read_u32be();
    let min_opcode = r.read_u32be();
    let max_opcode = r.read_u32be();
    let n_labels = r.read_u32be();
    let n_funs = r.read_u32be();
    println!("Code section version {}, opcodes {}-{}, labels: {}, funs: {}",
      code_ver, min_opcode, max_opcode, n_labels, n_funs);

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
        fun_atom, arity, code_pos, index, nfree, ouniq
      })
    }
  }

  fn load_line_info(&mut self, r: &mut BinaryReader) {
    let version = r.read_u32be(); // must match emulator version 0
    let flags = r.read_u32be();
    let n_line_instr = r.read_u32be();
    let n_line_refs = r.read_u32be();
    let n_filenames = r.read_u32be();
    let mut fname_index = 0u32;

    for _i in 0..n_line_refs {
      match compact_term::read(r).unwrap() {
        FTerm::SmallInt(w) => {
          // self.linerefs.push((fname_index, w));
        },
        FTerm::Atom(a) => fname_index = a as u32,
        other => panic!("{}Unexpected data in line info section: {:?}",
                        module(), other)
      }
    }

    for _i in 0..n_filenames {
      let name_size = r.read_u16be();
      let fstr = r.read_str_utf8(name_size as Word);
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
    let mut fun = function::Function::new();

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

      println!("Opcode {} {:?}", op, args);

      match op {
        // add nothing for label, but record its location
        x if x == gen_op::OPCODE::Label as u8 => {
          if let FTerm::Int_(f) = args[0] {
            // Store weak ptr to function and code offset to this label
            let floc = LLabel {
              fun: function::make_weak(&fun),
              offset: fun.borrow().code.len(),
            };
            self.labels.insert(f, floc);
          } else { panic_postprocess_instr(op, &args,0); }
        },

        // add nothing for line, but TODO: Record line contents
        x if x == gen_op::OPCODE::Line as u8 => {},

        x if x == gen_op::OPCODE::FuncInfo as u8 => {
          // arg[0] mod name, arg[1] fun name, arg[2] arity
          self.funarity = FunArity {
            f: args[1].to_lterm(),
            arity: args[2].loadtime_word() as Arity
          };
          // function finished, take it
          self.commit_fun(fun.clone());
          fun = function::Function::new();
        },

        _ => { // else
          let code = &mut fun.borrow_mut().code;
          code.push(op as Word);
          for a in args {
            if let FTerm::ExtList_(ref jtab) = a {
              for tmp in a.to_lterm_vec() {
                code.push(tmp.raw());
              }
            } else {
              code.push(a.to_lterm().raw());
            }
          }
        }
      }
    }
    // final function finished also take it
    self.commit_fun(fun);
  }

  /// Store the fun, which is probably completed loading, into the module
  /// dictionary.
  fn commit_fun(&mut self, fun: function::Ptr) {
    fun.borrow_mut().funarity = self.funarity.clone();
    let k = (self.funarity.f, self.funarity.arity);
    self.vm_funs.insert(k, fun);
    ()
  }
} // impl

fn panic_postprocess_instr(op: u8, args: &Vec<FTerm>, argi: Word) {
  panic!("{}Opcode {} the arg #{} in {:?} is bad", module(), op, argi, args)
}
