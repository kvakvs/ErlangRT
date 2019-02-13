use crate::{emulator::process::Process, term::lterm::LTerm};
use std::collections::HashMap;

pub struct ProcessRegistry {
  /// Dict of pids to process boxes
  pid_to_proc: HashMap<LTerm, Process>,
  name_to_pidport: HashMap<LTerm, LTerm>,
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
  pub fn insert(&mut self, pid: LTerm, proc: Process) {
    self.pid_to_proc.insert(pid, proc);
  }

  #[inline]
  pub fn remove(&mut self, pid: LTerm) {
    self.pid_to_proc.remove(&pid);
  }

  #[inline]
  pub fn count(&self) -> usize {
    self.pid_to_proc.len()
  }

  /// Borrow a read-only process, if it exists. Return `None` if we are sorry.
  #[inline]
  pub fn lookup_pid(&self, pid: LTerm) -> Option<&Process> {
    assert!(pid.is_local_pid());
    self.pid_to_proc.get(&pid)
  }

  /// Borrow a mutable process, if it exists. Return `None` if we are sorry.
  #[inline]
  pub fn lookup_pid_mut(&mut self, pid: LTerm) -> Option<&mut Process> {
    assert!(pid.is_local_pid());
    self.pid_to_proc.get_mut(&pid)
  }


  /// Find a process and instead of borrowing return a pointer to it.
  #[inline]
  #[allow(dead_code)]
  pub fn unsafe_lookup_pid(&self, pid: LTerm) -> *const Process {
    assert!(pid.is_local_pid());
    match self.pid_to_proc.get(&pid) {
      Some(p) => p as *const Process,
      None => core::ptr::null(),
    }
  }

  /// Find a process and instead of borrowing return a mutable pointer to it.
  #[inline]
  pub fn unsafe_lookup_pid_mut(&mut self, pid: LTerm) -> *mut Process {
    assert!(pid.is_local_pid());
    match self.pid_to_proc.get_mut(&pid) {
      Some(p) => p as *mut Process,
      None => core::ptr::null_mut(),
    }
  }

  /// Query contents of the name-to-pid/port table
  pub fn find_registered(&self, name: LTerm) -> Option<LTerm> {
    self.name_to_pidport.get(&name).cloned()
  }

  /// Add contents of the name-to-pid/port table, no check is made for whether
  /// the value is new, will overwrite.
  pub fn register_name(&mut self, name: LTerm, pid_or_port: LTerm) {
    self.name_to_pidport.insert(name, pid_or_port);
  }
}
