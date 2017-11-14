//! Functions to manipulate an LTerm as an Erlang Pid (immediate or a box)
//! either local or external. Part of LTerm impl.

//use rt_defs::Word;
use term::immediate;


pub trait PidAspect {
  /// Check whether a value is any kind of process identifier (pid).
  fn is_pid(&self) -> bool { self.is_local_pid() || self.is_external_pid() }

  fn is_local_pid(&self) -> bool;

  fn is_external_pid(&self) -> bool { false }
}


impl PidAspect for super::LTerm {
  /// Check whether a value is a local pid.
  #[inline]
  fn is_local_pid(&self) -> bool {
    immediate::is_pid_raw(self.value)
  }
}
