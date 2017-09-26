use std::collections::BTreeMap;
use std::boxed::Box;
use std::vec::Vec;

use term::Term;
use types::Word;
use process;
//use term::immediate;

//
// VM environment, heaps, atoms, tables, processes all goes here
//
pub struct VM {
  // Direct mapping string to atom index
  atoms: BTreeMap<Box<String>, Word>,
  // Reverse mapping atom index to string (sorted by index)
  atoms_r: Vec<Box<String>>,

  pid_counter: Word,
  processes: BTreeMap<Term, Box<process::Process>>,
}

impl VM {
  pub fn new() -> VM {
    VM {
      atoms: BTreeMap::new(),
      atoms_r: Vec::new(),
      pid_counter: 0,
      processes: BTreeMap::new(),
    }
  }

  // Allocate new atom in the atom table or find existing. Pack atom index as atom immediate2
  pub fn find_or_create_atom(&mut self, val: &String) -> Term {
    if self.atoms.contains_key(val) {
      return Term::make_atom(self.atoms[val]);
    }

    let index = self.atoms_r.len();

    // Ultra ugly: TODO
    let boxval0 = Box::into_raw(Box::new(val.to_string()));
    let boxval1 = unsafe { Box::from_raw(boxval0) };
    let boxval2 = unsafe { Box::from_raw(boxval0) };

    self.atoms.insert(boxval1, index);
    self.atoms_r.push(boxval2 );

    Term::make_atom(index)
  }

  pub fn create_process(&mut self, parent: Term) {
    let pid_c = self.pid_counter;
    self.pid_counter += 1;
    let pid = Term::make_pid(pid_c);
    process::Process::new(pid, parent);
    ()
  }
}