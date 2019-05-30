use crate::{emulator::gen_atoms, term::Term};
use core::fmt;

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
#[allow(dead_code)]
pub enum ExceptionType {
  Panic, // ignore catches
  Throw,
  Error,
  Exit,
}

impl ExceptionType {
  pub fn to_atom(self) -> Term {
    match self {
      ExceptionType::Panic => gen_atoms::NIF_ERROR, // todo: populate panic atom
      ExceptionType::Throw => gen_atoms::THROW,
      ExceptionType::Error => gen_atoms::ERROR,
      ExceptionType::Exit => gen_atoms::EXIT,
    }
  }
}

impl fmt::Display for ExceptionType {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match *self {
      ExceptionType::Exit => write!(f, "<exit>"),
      ExceptionType::Throw => write!(f, "<throw>"),
      ExceptionType::Error => write!(f, "<error>"),
      ExceptionType::Panic => write!(f, "<panic>"),
    }
  }
}
