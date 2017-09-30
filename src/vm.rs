//!
//! Implements virtual machine, as a collection of processes and their
//! registrations, schedulers, ETS tables and atom table etc.
//!
use std::boxed::Box;
use std::collections::BTreeMap;
use std::vec::Vec;

use code_srv;
use mfa;
use process::Process;
use rterror;
use term::Term;
use types::Word;

fn module() -> &'static str { "vm: " }

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
  pub fn atom(&mut self, val: &'a str) -> Term {
    if self.atoms.contains_key(val) {
      return Term::make_atom(self.atoms[val]);
    }

    let index = self.atoms_r.len();
    self.atoms.entry(val).or_insert(index);
    self.atoms_r.push(val);

    Term::make_atom(index)
  }

  // Spawn a new process, create a new pid, register the process and jump to the MFA
  pub fn create_process(&mut self, parent: Term, mfa: &mfa::MFArgs) -> Term {
    let pid_c = self.pid_counter;
    self.pid_counter += 1;
    let pid = Term::make_pid(pid_c);
    let mut p = Process::new(self, pid, parent, mfa).unwrap();
    self.processes.insert(pid, p);
    pid
  }

  /// Run the VM loop (one time slice), call this repeatedly to run forever.
  /// Returns: false if VM quit, true if can continue
  pub fn tick(&mut self) -> bool {
    true
  }

  pub fn atom_to_str(&self, atom: Term) -> String {
    assert!(atom.is_atom());
    self.atoms_r[atom.atom_index()].to_string()
  }

  /// Mutable lookup, will load module if lookup fails the first time
  pub fn code_lookup(&mut self,
                     mfa: &mfa::IMFArity) -> Result<code_srv::InstrPointer, rterror::Error> {
    // Try lookup once, then load if not found
    match self.code_srv.lookup(mfa) {
      Some(ip) => return Ok(ip),
      None => {
        let filename = self.atom_to_str(mfa.get_mod());
        match self.code_srv.load(&filename) {
          Ok(_ok2) => (),
          Err(er2) => return Err(er2)
        }
      }
    };
    // Try lookup again
    match self.code_srv.lookup(mfa) {
      Some(ip) => Ok(ip),
      None => {
        let mod_str = self.atom_to_str(mfa.get_mod());
        let fun_str = self.atom_to_str(mfa.get_fun());
        let msg = format!("{}Func undef: {}:{}/{}",
                          module(), mod_str, fun_str, mfa.get_arity());
        Err(rterror::Error::CodeLoadingFailed(msg))
      }
    }
  }
}