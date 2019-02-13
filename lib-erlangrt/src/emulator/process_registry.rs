use crate::{emulator::process::Process, term::lterm::LTerm};
use std::collections::HashMap;

pub struct ProcessRegistry {
  /// Dict of pids to process boxes
  reg: HashMap<LTerm, Process>,
}

impl ProcessRegistry {
  pub fn new() -> Self {
    Self {
      reg: HashMap::new(),
    }
  }

  /// Register a process `proc_` in the process table and also queue it for
  /// execution. This is invoked by vm when a new process is spawned.
  #[inline]
  pub fn insert(&mut self, pid: LTerm, proc: Process) {
    self.reg.insert(pid, proc);
  }

  #[inline]
  pub fn remove(&mut self, pid: LTerm) {
    self.reg.remove(&pid);
  }

  #[inline]
  pub fn count(&self) -> usize {
    self.reg.len()
  }

  /// Borrow a read-only process, if it exists. Return `None` if we are sorry.
  #[inline]
  pub fn lookup_pid(&self, pid: LTerm) -> Option<&Process> {
    assert!(pid.is_local_pid());
    self.reg.get(&pid)
  }

  /// Borrow a mutable process, if it exists. Return `None` if we are sorry.
  #[inline]
  pub fn lookup_pid_mut(&mut self, pid: LTerm) -> Option<&mut Process> {
    assert!(pid.is_local_pid());
    self.reg.get_mut(&pid)
  }


  /// Find a process and instead of borrowing return a pointer to it.
  #[inline]
  #[allow(dead_code)]
  pub fn unsafe_lookup_pid(&self, pid: LTerm) -> *const Process {
    assert!(pid.is_local_pid());
    match self.reg.get(&pid) {
      Some(p) => p as *const Process,
      None => core::ptr::null(),
    }
  }

  /// Find a process and instead of borrowing return a mutable pointer to it.
  #[inline]
  pub fn unsafe_lookup_pid_mut(&mut self, pid: LTerm) -> *mut Process {
    assert!(pid.is_local_pid());
    match self.reg.get_mut(&pid) {
      Some(p) => p as *mut Process,
      None => core::ptr::null_mut(),
    }
  }
}
