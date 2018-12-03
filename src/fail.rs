//!
//! Generic errors used everywhere in the VM runtime.
//!
use beam::compact_term::CTError;
use rt_util::bin_reader;
use rt_util::ext_term_format;
use emulator::heap::HeapError;

use std::convert::From;
use rt_defs::ExceptionType;
use term::lterm::LTerm;


// TODO: Rename to RTError-something
#[derive(Debug)]
pub enum Error {
  FileNotFound(String),
  ReadExternalTerm(ext_term_format::ETFError),

  //--- Code loading ---
  CodeLoading(bin_reader::ReadError),
  CodeLoadingFailed(String),
  CodeLoadingCompactTerm(CTError),
  //PrematureEOF,

  //--- Code server, lookups ---
  ModuleNotFound(String),
  FunctionNotFound(String),
  BifNotFound(String),

  //--- Memory allocation events and errors ---
  AtomNotExist(String),
  HeapError(HeapError),
  //StackIndexRange,

  //--- VM Checks --
  Exception(ExceptionType, LTerm), // type, value
  TermIsNotABoxed,
  BoxedIsNotAClosure,
  BoxedIsNotAnImport,
  BoxedIsNotATuple,
  HeapObjNotAType, // remove
}


impl From<bin_reader::ReadError> for Error {
  fn from(e: bin_reader::ReadError) -> Self { Error::CodeLoading(e) }
}

impl From<ext_term_format::ETFError> for Error {
  fn from(e: ext_term_format::ETFError) -> Self { Error::ReadExternalTerm(e) }
}

impl From<HeapError> for Error {
  fn from(e: HeapError) -> Self { Error::HeapError(e) }
}


/// A templated error type based on `fail::Error`.
// TODO: Rename to RTResult-something
pub type Hopefully<T> = Result<T, Error>;

//impl fmt::Debug for Error {
//  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//    match self {
//      &Error::FileNotFound(ref filename) =>
//        return write!(f, "File not found: {}", filename)
//    }
//    write!(f, "Some internal error")
//  }
//}
