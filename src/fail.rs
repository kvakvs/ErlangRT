//!
//! Generic errors used everywhere in the VM runtime.
//!
use beam::compact_term::CTError;
use rt_util::bin_reader;

use std::convert::From;


#[derive(Debug)]
pub enum Error {
  FileNotFound(String),
  ReadExternalTerm(String),

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
  HeapIsFull,
  StackIndexRange,
}


impl From<bin_reader::ReadError> for Error {
  fn from(br_err: bin_reader::ReadError) -> Self { Error::CodeLoading(br_err) }
}


/// A templated error type based on `fail::Error`.
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