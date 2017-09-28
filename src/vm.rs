use std::collections::BTreeMap;
use std::boxed::Box;
use std::vec::Vec;

use term::Term;
use types::Word;
use process::Process;
//use term::immediate;
use mfargs::MFArgs;

//
// VM environment, heaps, atoms, tables, processes all goes here
//
pub struct VM<'a> {
  // Direct mapping string to atom index
  atoms: BTreeMap<&'a str, Word>,
  // Reverse mapping atom index to string (sorted by index)
  atoms_r: Vec<&'a str>,

  // Pid counter increments every time a new process is spawned
  pid_counter: Word,
  // Dict of pids to process boxes
  processes: BTreeMap<Term, Process>,
}

impl<'a> VM<'a> {
  pub fn new() -> VM<'a> {
    VM {
      atoms: BTreeMap::new(),
      atoms_r: Vec::new(),
      pid_counter: 0,
      processes: BTreeMap::new(),
    }
  }

  // Allocate new atom in the atom table or find existing. Pack atom index as atom immediate2
  pub fn find_or_create_atom(&mut self, val: &'a str) -> Term {
    if self.atoms.contains_key(val) {
      return Term::make_atom(self.atoms[val]);
    }

    let index = self.atoms_r.len();
    self.atoms.entry(val).or_insert(index);
    self.atoms_r.push(val);

    Term::make_atom(index)
  }

  // Spawn a new process, create a new pid, register the process and jump to the MFA
  pub fn create_process(&mut self, parent: Term, mfa: &MFArgs) -> Term {
    let pid_c = self.pid_counter;
    self.pid_counter += 1;
    let pid = Term::make_pid(pid_c);
    let mut p = Process::new(pid, parent);
    p.jump(self, mfa);
    self.processes.insert(pid, p);
    pid
  }
}