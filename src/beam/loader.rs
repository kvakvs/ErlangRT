use bytes::Bytes;
use std::path::PathBuf;

use rterror;
use types::Word;
use util::reader;

pub struct Loader {
  atom_tab: Vec<String>,
}

impl Loader {
  /// Construct a new loader state.
  pub fn new() -> Loader {
    Loader { atom_tab: Vec::new() }
  }

  /// Loading the module. Validate the header and iterate over sections,
  /// then finalize by committing the changes to the VM.
  pub fn load(&mut self, fname: &PathBuf) -> Result<(), rterror::Error> {
    let mut r = reader::Reader::new(fname);

    // Parse header and check file FOR1 signature
    let hdr1 = Bytes::from(&b"FOR1"[..]);
    r.ensure_bytes(&hdr1)?;

    let beam_sz = r.read_u32be();

    // Check BEAM signature
    let hdr2 = Bytes::from(&b"BEAM"[..]);
    r.ensure_bytes(&hdr2)?;

    while true {
      let chunk_h = r.read_str(4);
      let chunk_sz = r.read_u32be();

      println!("Chunk {}", chunk_h);
      if "AtU8" == chunk_h {
        self.load_atoms(&mut r)
      }
      break;
    }

    Ok(())
  }

  /// Approaching AtU8 or Atom section, read the section and populate atoms
  /// table in the Loader state.
  /// The format is: "Atom"|"AtU8", u32/big count { u8 length, "atomname" }
  fn load_atoms(&mut self, r: &mut reader::Reader) {
    let n_atoms = r.read_u32be();
    for i in 1..n_atoms {
      let atom_bytes = r.read_u8();
      let atom_text = r.read_str(atom_bytes as Word);
      self.atom_tab.push(atom_text);
    }
  }
}
