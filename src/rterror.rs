//!
//! Generic errors used everywhere in the VM runtime.
//!
use beam::compact_term::CTError;

#[derive(Debug)]
pub enum Error {
  FileNotFound(String),
  CodeLoadingFailed(String),
  CodeLoadingPrematureEOF,
  CodeLoadingCompactTerm(CTError)
}

//impl fmt::Debug for Error {
//  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//    match self {
//      &Error::FileNotFound(ref filename) =>
//        return write!(f, "File not found: {}", filename)
//    }
//    write!(f, "Some internal error")
//  }
//}