use crate::{
  beam::{
    gen_op,
    loader::{
      compact_term,
      load_time_structs::{LtExport, LtFun, LtImport},
    },
  },
  defs,
  emulator::heap::{Heap, DEFAULT_LIT_HEAP},
  fail::{RtErr, RtResult},
  rt_util::{
    bin_reader::{BinaryReader, ReadError},
    ext_term_format as etf,
  },
  term::{lterm::Term, term_builder::TermBuilder},
};
use bytes::Bytes;
use compress::zlib;
use std::{
  io::{Cursor, Read},
  path::PathBuf,
};

fn module() -> &'static str {
  "beam/file: "
}

pub struct BeamFile {
  /// Raw atoms loaded from BEAM module as strings
  pub atoms: Vec<String>,
  pub imports: Vec<LtImport>,
  pub exports: Vec<LtExport>,
  pub locals: Vec<LtExport>,
  pub lambdas: Vec<LtFun>,
  /// Temporary storage for loaded code, will be parsed in stage 2
  pub code: Vec<u8>,

  /// Literal table decoded into friendly terms (does not use process heap).
  pub lit_tab: Vec<Term>,

  /// A place to allocate larger lterms (literal heap)
  pub lit_heap: Heap,

  /// Proplist of module attributes as loaded from "Attr" section
  mod_attrs: Term,

  /// Compiler flags as loaded from "Attr" section
  compiler_info: Term,
}

impl BeamFile {
  fn new() -> Self {
    Self {
      atoms: Vec::new(),
      imports: Vec::new(),
      exports: Vec::new(),
      locals: Vec::new(),
      lambdas: Vec::new(),
      code: Vec::new(),

      lit_tab: Vec::new(),
      lit_heap: Heap::new(DEFAULT_LIT_HEAP),
      mod_attrs: Term::nil(),
      compiler_info: Term::nil(),
    }
  }

  /// Loading the module. Validate the header and iterate over sections,
  /// then call `load_stage2()` to apply changes to the VM, and then finalize
  /// it by calling `load_finalize()` which will return you a module object.
  pub fn read_chunks(fname: &PathBuf) -> RtResult<BeamFile> {
    let mut beam_file = Self::new();

    // Prebuffered BEAM file should be released as soon as the initial phase
    // is done.
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
        Err(e) => return Err(RtErr::ReadError(e)),
      };
      let chunk_sz = r.read_u32be();
      let pos_begin = r.pos();

      // println!("Chunk {}", chunk_h);
      match chunk_h.as_ref() {
        "Atom" => beam_file.load_atoms_latin1(&mut r),
        "Attr" => beam_file.load_attributes(&mut r)?,
        "AtU8" => beam_file.load_atoms_utf8(&mut r),
        "CInf" => beam_file.load_compiler_info(&mut r)?,
        "Code" => beam_file.load_code(&mut r, chunk_sz as defs::Word)?,
        "ExpT" => beam_file.exports = beam_file.load_exports(&mut r),
        "FunT" => beam_file.load_fun_table(&mut r),
        "ImpT" => beam_file.load_imports(&mut r),
        "Line" => beam_file.load_line_info(&mut r)?,
        "LitT" => beam_file.load_literals(&mut r, chunk_sz as defs::Word),
        // LocT same format as ExpT, but for local functions
        "LocT" => beam_file.locals = beam_file.load_exports(&mut r),

        "Dbgi" | // skip debug info
        "StrT" | // skip strings TODO load strings?
        "Abst" => r.skip(chunk_sz as usize), // skip abstract code

        other => {
          let msg = format!("{}Unexpected chunk: {}", module(), other);
          return Err(RtErr::CodeLoadingFailed(msg))
        }
      }

      // The next chunk is aligned at 4 bytes
      let aligned_sz = 4 * ((chunk_sz + 3) / 4);
      r.seek(pos_begin + aligned_sz as usize);
    }

    Ok(beam_file)
  }

  /// Approaching AtU8 section, populate atoms table in the Loader state.
  /// The format is: "Atom"|"AtU8", u32/big count { u8 length, "atomname" }.
  /// Formats are absolutely compatible except that Atom is latin-1
  fn load_atoms_utf8(&mut self, r: &mut BinaryReader) {
    let n_atoms = r.read_u32be();
    self.atoms.reserve(n_atoms as usize);
    for _i in 0..n_atoms {
      let atom_bytes = r.read_u8();
      let atom_text = r.read_str_utf8(atom_bytes as defs::Word).unwrap();
      self.atoms.push(atom_text);
    }
  }

  /// Approaching Atom section, populate atoms table in the Loader state.
  /// The format is: "Atom"|"AtU8", u32/big count { u8 length, "atomname" }.
  /// Same as `load_atoms_utf8` but interprets strings per-character as latin-1
  fn load_atoms_latin1(&mut self, r: &mut BinaryReader) {
    let n_atoms = r.read_u32be();
    self.atoms.reserve(n_atoms as usize);
    for _i in 0..n_atoms {
      let atom_bytes = r.read_u8();
      let atom_text = r.read_str_latin1(atom_bytes as defs::Word).unwrap();
      self.atoms.push(atom_text);
    }
  }

  /// Read Attr section: two terms (module attributes and compiler info) encoded
  /// as external term format.
  fn load_attributes(&mut self, r: &mut BinaryReader) -> RtResult<()> {
    let mut tb = TermBuilder::new(&mut self.lit_heap);
    self.mod_attrs = etf::decode(r, &mut tb)?;
    Ok(())
  }

  fn load_compiler_info(&mut self, r: &mut BinaryReader) -> RtResult<()> {
    let mut tb = TermBuilder::new(&mut self.lit_heap);
    self.compiler_info = etf::decode(r, &mut tb)?;
    Ok(())
  }

  /// Load the `Code` section
  fn load_code(&mut self, r: &mut BinaryReader, chunk_sz: defs::Word) -> RtResult<()> {
    let _code_ver = r.read_u32be();
    let _min_opcode = r.read_u32be();
    let max_opcode = r.read_u32be();
    let _n_labels = r.read_u32be();
    let _n_funs = r.read_u32be();
    // println!("Code section version {}, opcodes {}-{}, labels: {}, funs: {}",
    //  code_ver, min_opcode, max_opcode, n_labels, n_funs);

    if max_opcode > gen_op::OPCODE_MAX.get() as u32 {
      let msg = "BEAM file comes from a never and unsupported OTP version".to_string();
      return Err(RtErr::CodeLoadingFailed(msg));
    }

    self.code = r.read_bytes(chunk_sz - 20).unwrap();
    Ok(())
  }

  /// Read the imports table.
  /// Format is u32/big count { modindex: u32, funindex: u32, arity: u32 }
  fn load_imports(&mut self, r: &mut BinaryReader) {
    let n_imports = r.read_u32be();
    self.imports.reserve(n_imports as usize);
    for _i in 0..n_imports {
      let imp = LtImport {
        mod_atom_i: r.read_u32be() as usize,
        fun_atom_i: r.read_u32be() as usize,
        arity: r.read_u32be() as defs::Arity,
      };
      self.imports.push(imp);
    }
  }

  /// Read the exports or local functions table (same format).
  /// Format is u32/big count { funindex: u32, arity: u32, label: u32 }
  fn load_exports(&mut self, r: &mut BinaryReader) -> Vec<LtExport> {
    let n_exports = r.read_u32be();
    let mut exports = Vec::new();
    exports.reserve(n_exports as usize);
    for _i in 0..n_exports {
      let exp = LtExport {
        fun_atom_i: r.read_u32be() as usize,
        arity: r.read_u32be() as defs::Arity,
        label: r.read_u32be() as usize,
      };
      exports.push(exp);
    }
    exports
  }

  fn load_fun_table(&mut self, r: &mut BinaryReader) {
    let n_funs = r.read_u32be();
    self.lambdas.reserve(n_funs as usize);
    for _i in 0..n_funs {
      let fun_atom = r.read_u32be() as usize;
      let arity = r.read_u32be() as usize;
      let code_pos = r.read_u32be() as usize;
      let index = r.read_u32be() as usize;
      let nfrozen = r.read_u32be() as usize;
      let ouniq = r.read_u32be() as usize;
      self.lambdas.push(LtFun {
        fun_atom_i: fun_atom,
        arity: arity as defs::Arity,
        code_pos,
        index,
        nfrozen,
        ouniq,
      })
    }
  }

  fn load_line_info(&mut self, r: &mut BinaryReader) -> RtResult<()> {
    let _version = r.read_u32be(); // must match emulator version 0
    let _flags = r.read_u32be();
    let _n_line_instr = r.read_u32be();
    let n_line_refs = r.read_u32be();
    let n_filenames = r.read_u32be();
    let mut _fname_index = 0u32;

    for _i in 0..n_line_refs {
      let val = compact_term::read(r)?;
      if val.is_small() {
          // self.linerefs.push((_fname_index, w));
      } else if val.is_atom() {
        // _fname_index = a as u32
      } else {
        panic!("{}Unexpected data in line info section: {}", module(), val)
      }
    }

    for _i in 0..n_filenames {
      let name_size = r.read_u16be();
      let _fstr = r.read_str_utf8(name_size as defs::Word);
    }
    Ok(())
  }

  /// Given the `r`, reader positioned on the contents of "LitT" chunk,
  /// decompress it and feed into `self.decode_literals/1`
  fn load_literals(&mut self, r: &mut BinaryReader, chunk_sz: defs::Word) {
    // Red uncompressed size and reserve some memory
    let uncomp_sz = r.read_u32be();
    let mut inflated = Vec::<u8>::new();
    inflated.reserve(uncomp_sz as usize);

    // Deduce the 4 bytes uncomp_sz
    let deflated = r.read_bytes(chunk_sz - 4).unwrap();
    // dump_vec(&deflated);

    // Decompress deflated literal table
    let iocursor = Cursor::new(&deflated);
    zlib::Decoder::new(iocursor)
      .read_to_end(&mut inflated)
      .unwrap();
    assert_eq!(
      inflated.len(),
      uncomp_sz as usize,
      "{}LitT inflate failed",
      module()
    );

    // Parse literal table
    // dump_vec(&inflated);
    self.decode_literals(inflated);
  }

  /// Given `inflated`, the byte contents of literal table, read the u32/big
  /// `count` and for every encoded term skip u32 and parse the external term
  /// format. Boxed values will go into the `self.lit_heap`.
  fn decode_literals(&mut self, inflated: Vec<u8>) {
    // dump_vec(&inflated);

    // Decode literals into literal heap here
    let mut r = BinaryReader::from_bytes(inflated);
    let count = r.read_u32be();
    self.lit_tab.reserve(count as usize);

    for _i in 0..count {
      // size should match actual consumed ETF bytes so can skip it here
      let _size = r.read_u32be();

      let mut tb = TermBuilder::new(&mut self.lit_heap);

      // TODO: Instead of unwrap return error and possibly log/return error location too?
      let literal = etf::decode(&mut r, &mut tb).unwrap();

      self.lit_tab.push(literal);
    }
  }
}
