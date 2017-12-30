//! Functions to manipulate an `LTerm` as an boxed pointer to an Erlang Port
//! either local or external. Part of `LTerm` impl.
//! Do not import this file directly, use `use term::lterm::*;` instead.

//use rt_defs::Word;
//use term::immediate;


pub trait PortAspect {
  /// Check whether a value is any kind of port.
  fn is_port(&self) -> bool { self.is_local_port() || self.is_external_port() }

  fn is_local_port(&self) -> bool { false }

  fn is_external_port(&self) -> bool { false }
}


impl PortAspect for super::LTerm {
}
