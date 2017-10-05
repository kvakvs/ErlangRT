//!
//! Generic errors used everywhere in the VM runtime.
//!
use beam::compact_term::CTError;

#[derive(Debug)]
pub enum Error {
  FileNotFound(String),
  //--- Code loading ---
  CodeLoadingFailed(String),
  CodeLoadingPrematureEOF,
  CodeLoadingCompactTerm(CTError),
  //--- Code server, lookups ---
  ModuleNotFound(String),
  FunctionNotFound(String),
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