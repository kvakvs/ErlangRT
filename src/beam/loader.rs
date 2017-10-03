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
use emulator::mfa::Arity;
use emulator::module;
use emulator::vm::VM;
use emulator::gen_op;
use rterror;
use term::friendly;
use term::low_level;
use defs::{Word, Integral};
use util::bin_reader;

pub fn module() -> &'static str { "BEAM loader: " }

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
  vm_atoms: Vec<low_level::Term>,
  vm_funs: BTreeMap<low_level::Term, function::Ptr>,
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
    }
  }

  /// Loading the module. Validate the header and iterate over sections,
  /// then call `load_stage2()` to apply changes to the VM, and then finalize
  /// it by calling `load_finalize()` which will return you a module object.
  pub fn load(&mut self, fname: &PathBuf) -> Result<(), rterror::Error>
  {
    // Prebuffered BEAM file should be released as soon as the initial phase
    // is done. TODO: [Performance] Use memmapped file?
    let mut r = bin_reader::BinaryReader::from_file(fname);

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
    Ok(newmod)
  }

  //============================================================================

  /// Approaching AtU8 section, populate atoms table in the Loader state.
  /// The format is: "Atom"|"AtU8", u32/big count { u8 length, "atomname" }.
  /// Formats are absolutely compatible except that Atom is latin-1
  fn load_atoms_utf8(&mut self, r: &mut bin_reader::BinaryReader) {
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
  fn load_atoms_latin1(&mut self, r: &mut bin_reader::BinaryReader) {
    let n_atoms = r.read_u32be();
    self.raw_atoms.reserve(n_atoms as usize);
    for i in 0..n_atoms {
      let atom_bytes = r.read_u8();
      let atom_text = r.read_str_latin1(atom_bytes as Word).unwrap();
      self.raw_atoms.push(atom_text);
    }
  }

  /// Load the `Code` section
  fn load_code(&mut self, r: &mut bin_reader::BinaryReader, chunk_sz: Word) {
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
  fn load_imports(&mut self, r: &mut bin_reader::BinaryReader) {
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
  fn load_exports(&mut self, r: &mut bin_reader::BinaryReader) -> Vec<LExport> {
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

  fn load_fun_table(&mut self, r: &mut bin_reader::BinaryReader) {
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

  fn load_line_info(&mut self, r: &mut bin_reader::BinaryReader) {
    let version = r.read_u32be(); // must match emulator version 0
    let flags = r.read_u32be();
    let n_line_instr = r.read_u32be();
    let n_line_refs = r.read_u32be();
    let n_filenames = r.read_u32be();
    let mut fname_index = 0u32;

    for _i in 0..n_line_refs {
      match compact_term::read(r).unwrap() {
        friendly::Term::SmallInt(w) => {
          // self.linerefs.push((fname_index, w));
        },
        friendly::Term::Atom(a) => fname_index = a as u32,
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
    let mut r = bin_reader::BinaryReader::from_bytes(raw_code);

    // Writing code unpacked to words here
    let mut outp: Vec<Word> = Vec::new();

    while !r.eof() {
      let opcode = r.read_u8();
      let arity = gen_op::opcode_arity(opcode);

      // TODO: can store 3-7 bytes of good stuff in the same word!
      outp.push(opcode as Word);
      print!("op[{}] '{}' ", opcode, gen_op::opcode_name(opcode));

      for _i in 0..arity {
        let arg = compact_term::read(&mut r).unwrap();
        // TODO: Can possibly pack x/y/fp regs into the first opcode word
        print!("{:?} ", &arg);
        outp.push(self.postprocess_to_word(arg));
      }
      println!()
    }
  }

  /// Given some simple friendly::Term produce an encoded compact Word with it
  /// to be stored as an opcode argument.
  fn postprocess_to_word(&self, arg: friendly::Term) -> Word {
    match arg {
      friendly::Term::Int_(i) => low_level::Term::make_small(i).raw(),
      friendly::Term::Nil => low_level::Term::nil().raw(),
      friendly::Term::Atom_(a) => self.vm_atoms[a].raw(),
      friendly::Term::X_(x) => low_level::Term::make_xreg(x).raw(),
      _ => panic!("{}Don't know how to represent {:?} as a Word", module(), arg)
    }
  }
}
