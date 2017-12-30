//! Functions to manipulate an `LTerm` as an boxed pointer to an Erlang Fun.
//! Part of `LTerm` impl.
//! Do not import this file directly, use `use term::lterm::*;` instead.


pub trait FunAspect {
  /// Check whether a value is a boxed fun (a closure).
  fn is_fun(&self) -> bool { false }
}


impl FunAspect for super::LTerm {
}
