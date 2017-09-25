use std::collections::BTreeMap;
use std::boxed::Box;
use std::vec::Vec;

use term::Term;
use types::Word;

//
// VM environment, heaps, atoms, tables, processes all goes here
//
pub struct VM {
  atom_index: Word,
  // Direct mapping string to atom index
  atoms: BTreeMap<Box<String>, Word>,
  // Reverse mapping atom index to string (sorted by index)
  atoms_r: Vec<Box<String>>,
}

impl VM {
  pub fn new() -> VM {
    VM {
      atom_index: 0,
      atoms: BTreeMap::new(),
      atoms_r: Vec::new(),
    }
  }

  pub fn new_atom(&mut self, val: &String) -> Term {
    if self.atoms.contains_key(val) {
      return Term::new_atom(self.atoms[val]);
    }

    let index = self.atom_index;
    self.atom_index += 1;

    // Ultra ugly: TODO
    let boxval0 = Box::into_raw(Box::new(val.to_string()));
    let boxval1 = unsafe { Box::from_raw(boxval0) };
    let boxval2 = unsafe { Box::from_raw(boxval0) };

    self.atoms.insert(boxval1, index);
    self.atoms_r.push(boxval2 );

    Term::new_atom(index)
  }
}