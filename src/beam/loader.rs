use bytes::Bytes;
use std::path::PathBuf;

use rterror;
use types::Word;
use mfa::Arity;
use util::reader;

pub fn module() -> &'static str { "BEAM loader: " }

struct Import {
  mod_atom: u32,
  fun_atom: u32,
  arity: Arity,
}

struct Export {
  fun_atom: u32,
  arity: Arity,
  label: u32,
}

pub struct Loader {
  atom_tab: Vec<String>,
  imports: Vec<Import>,
  exports: Vec<Export>,
}

impl Loader {
  /// Construct a new loader state.
  pub fn new() -> Loader {
    Loader {
      atom_tab: Vec::new(),
      imports: Vec::new(),
      exports: Vec::new(),
    }
  }

  /// Loading the module. Validate the header and iterate over sections,
  /// then finalize by committing the changes to the VM.
  pub fn load(&mut self, fname: &PathBuf) -> Result<(), rterror::Error> {
    let mut r = reader::BinaryReader::new(fname);

    // Parse header and check file FOR1 signature
    let hdr1 = Bytes::from(&b"FOR1"[..]);
    r.ensure_bytes(&hdr1)?;

    let beam_sz = r.read_u32be();

    // Check BEAM signature
    let hdr2 = Bytes::from(&b"BEAM"[..]);
    r.ensure_bytes(&hdr2)?;

    while true {
      let chunk_h = r.read_str_latin1(4);
      let chunk_sz = r.read_u32be();

      println!("Chunk {}", chunk_h);
      match chunk_h.as_ref() {
        "AtU8" => self.load_atoms_utf8(&mut r),
        "Atom" => self.load_atoms_latin1(&mut r),
        "Code" => self.load_code(&mut r, chunk_sz as Word),
        "StrT" => r.skip(chunk_sz as Word),
        "ImpT" => self.load_imports(&mut r),
        "ExpT" => self.load_exports(&mut r),
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

  /// Approaching AtU8 section, populate atoms table in the Loader state.
  /// The format is: "Atom"|"AtU8", u32/big count { u8 length, "atomname" }.
  /// Formats are absolutely compatible except that Atom is latin-1
  fn load_atoms_utf8(&mut self, r: &mut reader::BinaryReader) {
    let n_atoms = r.read_u32be();
    for i in 0..n_atoms {
      let atom_bytes = r.read_u8();
      let atom_text = r.read_str_utf8(atom_bytes as Word);
      self.atom_tab.push(atom_text);
    }
  }

  /// Approaching Atom section, populate atoms table in the Loader state.
  /// The format is: "Atom"|"AtU8", u32/big count { u8 length, "atomname" }.
  /// Same as `load_atoms_utf8` but interprets strings per-character as latin-1
  fn load_atoms_latin1(&mut self, r: &mut reader::BinaryReader) {
    let n_atoms = r.read_u32be();
    self.atom_tab.reserve(n_atoms as usize);
    for i in 0..n_atoms {
      let atom_bytes = r.read_u8();
      let atom_text = r.read_str_latin1(atom_bytes as Word);
      self.atom_tab.push(atom_text);
    }
  }

  /// Load the `Code` section
  fn load_code(&mut self, r: &mut reader::BinaryReader, chunk_sz: Word) {
    let code_ver = r.read_u32be();
    let min_opcode = r.read_u32be();
    let max_opcode = r.read_u32be();
    let n_labels = r.read_u32be();
    let n_funs = r.read_u32be();
    println!("Code section version {}, opcodes {}-{}, labels: {}, funs: {}",
      code_ver, min_opcode, max_opcode, n_labels, n_funs);
    let code = r.read_bytes(chunk_sz - 20);
  }

  /// Read the imports table.
  /// Format is u32/big count { modindex: u32, funindex: u32, arity: u32 }
  fn load_imports(&mut self, r: &mut reader::BinaryReader) {
    let n_imports = r.read_u32be();
    self.imports.reserve(n_imports as usize);
    for i in 0..n_imports {
      let imp = Import {
        mod_atom: r.read_u32be(),
        fun_atom: r.read_u32be(),
        arity: r.read_u32be() as Arity,
      };
      self.imports.push(imp);
    }
  }

  /// Read the exports table.
  /// Format is u32/big count { funindex: u32, arity: u32, label: u32 }
  fn load_exports(&mut self, r: &mut reader::BinaryReader) {
    let n_exports = r.read_u32be();
    self.exports.reserve(n_exports as usize);
    for i in 0..n_exports {
      let exp = Export {
        fun_atom: r.read_u32be(),
        arity: r.read_u32be() as Arity,
        label: r.read_u32be(),
      };
      self.exports.push(exp);
    }
  }
}
