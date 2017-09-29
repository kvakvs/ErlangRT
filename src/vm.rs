use std::boxed::Box;
use std::collections::BTreeMap;
use std::vec::Vec;

use code_srv;
use mfargs;
use process::Process;
use rterror;
use term::Term;
use types::Word;

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

  code_srv: code_srv::CodeServer,
}

impl<'a> VM<'a> {
  pub fn new() -> VM<'a> {
    VM {
      atoms: BTreeMap::new(),
      atoms_r: Vec::new(),
      pid_counter: 0,
      processes: BTreeMap::new(),
      code_srv: code_srv::CodeServer::new()
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
  pub fn create_process(&mut self, parent: Term, mfa: &mfargs::MFArgs) -> Term {
    let pid_c = self.pid_counter;
    self.pid_counter += 1;
    let pid = Term::make_pid(pid_c);
    let mut p = Process::new(self, pid, parent, mfa).unwrap();
    self.processes.insert(pid, p);
    pid
  }

  // Run the VM loop (one time slice), call this repeatedly to run forever.
  // Returns: false if VM quit, true if can continue
  pub fn tick(&mut self) -> bool {
    true
  }

  pub fn atom_to_str(&self, atom: Term) -> String {
    assert!(atom.is_atom());
    self.atoms_r[atom.atom_index()].to_string()
  }

  pub fn code_lookup(&mut self,
                     mfa: &mfargs::IMFArity) -> Result<code_srv::InstrPointer, rterror::Error> {
    match self.code_srv.lookup(mfa) {
      Ok(ok1) => return Ok(ok1),
      Err(_er1) => {
        let filename = self.atom_to_str(mfa.get_mod());
        match self.code_srv.load(&filename) {
          Ok(_ok2) => (),
          Err(er2) => return Err(er2)
        }
      }
    };
    self.code_srv.lookup(mfa) // try again
  }
}