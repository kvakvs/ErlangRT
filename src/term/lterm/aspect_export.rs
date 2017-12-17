//! Functions to manipulate an `LTerm` as an boxed pointer to an Erlang Export.
//! Part of `LTerm` impl.

//use rt_defs::Word;
//use term::immediate;


pub trait ExportAspect {
  /// Check whether a value is a boxed export (M:F/Arity triple).
  fn is_export(&self) -> bool { false }
}


impl ExportAspect for super::LTerm {
}
