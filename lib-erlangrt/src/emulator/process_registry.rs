use crate::{emulator::process::Process, term::value::Term};
use std::collections::HashMap;

pub struct ProcessRegistry {
  /// Dict of pids to process boxes
  pid_to_proc: HashMap<Term, Process>,
  name_to_pidport: HashMap<Term, Term>,
}

impl ProcessRegistry {
  pub fn new() -> Self {
    Self {
      pid_to_proc: HashMap::new(),
      name_to_pidport: HashMap::new(),
    }
  }

  /// Register a process `proc_` in the process table and also queue it for
  /// execution. This is invoked by vm when a new process is spawned.
  #[inline]
  pub fn insert(&mut self, pid: Term, proc: Process) {
    self.pid_to_proc.insert(pid, proc);
  }

  #[inline]
  pub fn remove(&mut self, pid: Term) {
    self.pid_to_proc.remove(&pid);
  }

  #[inline]
  pub fn count(&self) -> usize {
    self.pid_to_proc.len()
  }

  /// Borrow a read-only process, if it exists. Return `None` if we are sorry.
  #[inline]
  pub fn lookup_pid(&self, pid: Term) -> Option<&Process> {
    debug_assert!(pid.is_local_pid());
    self.pid_to_proc.get(&pid)
  }

  /// Borrow a mutable process, if it exists. Return `None` if we are sorry.
  #[inline]
  pub fn lookup_pid_mut(&mut self, pid: Term) -> Option<&mut Process> {
    debug_assert!(pid.is_local_pid());
    self.pid_to_proc.get_mut(&pid)
  }


  /// Find a process and instead of borrowing return a pointer to it.
  #[inline]
  #[allow(dead_code)]
  pub fn unsafe_lookup_pid(&self, pid: Term) -> *const Process {
    debug_assert!(pid.is_local_pid());
    match self.pid_to_proc.get(&pid) {
      Some(p) => p as *const Process,
      None => core::ptr::null(),
    }
  }

  /// Find a process and instead of borrowing return a mutable pointer to it.
  #[inline]
  pub fn unsafe_lookup_pid_mut(&mut self, pid: Term) -> *mut Process {
    debug_assert!(pid.is_local_pid());
    match self.pid_to_proc.get_mut(&pid) {
      Some(p) => p as *mut Process,
      None => core::ptr::null_mut(),
    }
  }

  /// Query contents of the name-to-pid/port table
  pub fn find_registered(&self, name: Term) -> Option<Term> {
    self.name_to_pidport.get(&name).cloned()
  }

  /// Add contents of the name-to-pid/port table, no check is made for whether
  /// the value is new, will overwrite.
  pub fn register_name(&mut self, name: Term, pid_or_port: Term) {
    self.name_to_pidport.insert(name, pid_or_port);
  }
}
