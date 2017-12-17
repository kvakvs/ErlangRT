//! Functions to manipulate an `LTerm` as an boxed pointer to an Erlang Bignum.
//! Part of `LTerm` impl.

//use rt_defs::Word;
//use term::immediate;


pub trait BignumAspect {
  /// Check whether a value is a boxed bignum.
  fn is_bignum(&self) -> bool { false }
}


impl BignumAspect for super::LTerm {
}
