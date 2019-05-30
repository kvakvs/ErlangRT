//! Global Atom storage.
//! Global is ugly, but necessary for now. If we ever create more than 1 VM,
//! this will have to be shared somehow.

use crate::{
  defs::Word,
  emulator::gen_atoms,
  fail::{RtErr, RtResult},
  term::*,
};
use core::ptr;
use std::{
  collections::BTreeMap,
  sync::{Mutex, MutexGuard},
  u16,
};

/// Defines atom properties (length, compare helper integer)
pub struct Atom {
  /// Length of utf8-encoded atom name.
  pub len: u16,
  // /// Length of latin1-encoded atom otherwise -1
  // latin1_chars: i16,
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
    if !b.is_empty() {
      ord0 = u32::from(b[0]) << 24;
      if b.len() > 1 {
        ord0 |= u32::from(b[1]) << 16;
        if b.len() > 2 {
          ord0 |= u32::from(b[2]) << 8;
          if b.len() > 3 {
            ord0 |= u32::from(b[3]);
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
  atoms_by_str: Mutex<StrLookup>,

  /// Reverse mapping atom index to string (sorted by index)
  atoms_by_index: Mutex<IndexLookup>,
}

/// Stores atom lookup tables.
impl AtomStorage {
  pub fn add_init_atoms(&mut self) {
    let mut atoms = self.atoms_by_str.lock().unwrap();
    let mut atoms_r = self.atoms_by_index.lock().unwrap();

    for (index, ga) in gen_atoms::ATOM_INIT_NAMES.iter().enumerate() {
      let actual_i = AtomStorage::register_atom(&mut atoms, &mut atoms_r, ga);
      assert_eq!(index, actual_i);
    }
  }

  fn register_atom(
    atoms_by_str: &mut MutexGuard<StrLookup>,
    atoms_by_index: &mut MutexGuard<IndexLookup>,
    s: &str,
  ) -> Word {
    let index = atoms_by_index.len();
    atoms_by_str.insert(s.to_string(), index);
    atoms_by_index.push(Atom::new(s));
    index
  }
}

lazy_static! {
  static ref ATOMS: AtomStorage = {
    let mut storage = AtomStorage {
      atoms_by_str: Mutex::new(BTreeMap::new()),
      atoms_by_index: Mutex::new(Vec::new()),
    };
    storage.add_init_atoms();
    storage
  };
}

// Allocate new atom in the atom table or find existing. Pack the atom index
// as an immediate2 Term
pub fn from_str(val: &str) -> Term {
  let mut atoms = ATOMS.atoms_by_str.lock().unwrap();

  if atoms.contains_key(val) {
    return Term::make_atom(atoms[val]);
  }

  let mut atoms_r = ATOMS.atoms_by_index.lock().unwrap();

  let index = AtomStorage::register_atom(&mut atoms, &mut atoms_r, val);

  Term::make_atom(index)
}

pub fn to_str(a: Term) -> RtResult<String> {
  assert!(a.is_atom());
  let p = lookup(a);
  if p.is_null() {
    return Err(RtErr::AtomNotExist(format!("index {}", a.atom_index())));
  }
  Ok(unsafe { (*p).name.clone() })
}

/// Checks atom contents and returns true if it only contains alpanumeric
/// characters and underscores, and begins with a letter.
#[inline]
pub fn is_printable_atom(s: &str) -> bool {
  if s.len() == 0 {
    return false;
  }
  let mut s_chars = s.char_indices();
  let first = s_chars.next().unwrap();
  if !first.1.is_ascii_lowercase() {
    return false;
  }
  while let Some(c) = s_chars.next() {
    if !c.1.is_alphanumeric() && c.1 != '_' {
      return false;
    }
  }
  true
}

pub fn lookup(a: Term) -> *const Atom {
  assert!(a.is_atom());
  let atoms_r = ATOMS.atoms_by_index.lock().unwrap();
  let index = a.atom_index();
  if index >= atoms_r.len() {
    return ptr::null();
  }
  &atoms_r[index] as *const Atom
}
