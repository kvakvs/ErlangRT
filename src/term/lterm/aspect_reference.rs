//! Functions to manipulate an `LTerm` as an boxed pointer to an Erlang
//! Reference either local or external. Part of `LTerm` impl.
//! Do not import this file directly, use `use term::lterm::*;` instead.

//use rt_defs::Word;
//use term::immediate;


pub trait ReferenceAspect {
  /// Check whether a value is any kind of reference.
  fn is_ref(&self) -> bool { self.is_local_ref() || self.is_external_ref() }

  fn is_local_ref(&self) -> bool { false }

  fn is_external_ref(&self) -> bool { false }
}


impl ReferenceAspect for super::LTerm {
}
