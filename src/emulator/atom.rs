//! Global Atom storage.
//! Global is ugly, but necessary for now. If we ever create more than 1 VM,
//! this will have to be shared somehow.

use std::ptr;
use std::u16;
use std::collections::BTreeMap;
use std::sync::{Mutex, MutexGuard};

use fail::{Hopefully, Error};
use defs::Word;
use term::lterm::LTerm;
use emulator::gen_atoms;


/// Defines atom properties (length, compare helper integer)
pub struct Atom {
  /// Length of utf8-encoded atom name.
  pub len: u16,
  // /// Length of latin1-encoded atom otherwise -1
  //latin1_chars: i16,
  /// First 4 bytes used for comparisons
  pub ord0: u32,
  // TODO: Allocate these on atom heap or as a sequence of static blocks
  pub name: String,
}


impl Atom {
  /// Create and fill atom description structure, it will exist forever on the
  /// atom table in `AtomStorage`.
  pub fn new(s: &str) -> Atom {
    let b = s.as_bytes();
    let mut ord0 = 0u32;

    // This might be particularly ugly. Erlang/OTP does this by preallocating
    // a minimum of 4 bytes and taking from them unconditionally.
    if b.len() > 0 {
      ord0 = (b[0] as u32) << 24;
      if b.len() > 1 {
        ord0 |= (b[1] as u32) << 16;
        if b.len() > 2 {
          ord0 |= (b[2] as u32) << 8;
          if b.len() > 3 {
            ord0 |= b[3] as u32;
          }
        }
      }
    }

    assert!(s.len() <= u16::MAX as usize);
    Atom {
      len: s.len() as u16,
      ord0,
      name: s.to_string(),
    }
  }
}


/// A quick way to find an atom index by its string.
type StrLookup = BTreeMap<String, usize>;

/// A quick way to find an atom by its index.
type IndexLookup = Vec<Atom>;


/// Lookup table for atom to atom index and back. Declared static for use by
/// printing and atom loading facilities without having to pass the VM pointer
/// all the way down.
struct AtomStorage {
//  /// Ever growing array of const pointers to string blocks. Blocks are
//  /// allocated when the previous block is filled.
//  /// NOTE: By design these blocks are not movable, make movable maybe?
//  str_storage: Vec<*const u8>,

  /// Direct mapping string to atom index
  atoms: Mutex<StrLookup>,

  /// Reverse mapping atom index to string (sorted by index)
  atoms_r: Mutex<IndexLookup>,
}


/// Stores atom lookup tables.
impl AtomStorage {
  pub fn add_init_atoms(&mut self) {
    let mut atoms = self.atoms.lock().unwrap();
    let mut atoms_r = self.atoms_r.lock().unwrap();

    for (index, ga) in gen_atoms::ATOM_INIT_NAMES.iter().enumerate() {
      let actual_i = AtomStorage::register_atom(&mut atoms, &mut atoms_r, ga);
      assert_eq!(index, actual_i);
    }
  }


  fn register_atom(atoms: &mut MutexGuard<StrLookup>,
                   atoms_r: &mut MutexGuard<IndexLookup>,
                   s: &str) -> Word {
    let index = atoms_r.len();
    atoms.insert(s.to_string(), index);
    atoms_r.push(Atom::new(s));
    index
  }
}

lazy_static! {
  static ref ATOMS: AtomStorage = {
    let mut storage = AtomStorage {
      atoms: Mutex::new(BTreeMap::new()),
      atoms_r: Mutex::new(Vec::new()),
    };
    storage.add_init_atoms();
    storage
  };
}


// Allocate new atom in the atom table or find existing. Pack the atom index
// as an immediate2 Term
pub fn from_str(val: &str) -> LTerm {
  let mut atoms = ATOMS.atoms.lock().unwrap();

  if atoms.contains_key(val) {
    return LTerm::make_atom(atoms[val]);
  }

  let mut atoms_r = ATOMS.atoms_r.lock().unwrap();

  let index = AtomStorage::register_atom(
    &mut atoms, &mut atoms_r, val
  );

  LTerm::make_atom(index)
}


pub fn to_str(a: LTerm) -> Hopefully<String> {
  assert!(a.is_atom());
  let p = lookup(a);
  if p.is_null() {
    return Err(Error::AtomNotExist(format!("index {}", a.atom_index())))
  }
  Ok(unsafe { (*p).name.clone() })
}


pub fn lookup(a: LTerm) -> *const Atom {
  assert!(a.is_atom());
  let atoms_r = ATOMS.atoms_r.lock().unwrap();
  let index = a.atom_index();
  if index >= atoms_r.len() {
    return ptr::null()
  }
  &atoms_r[index] as *const Atom
}
