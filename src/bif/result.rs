//use std::convert::From;
use std::fmt;

//use fail;
use term::lterm::LTerm;
use rt_defs::{ExceptionType};


/// Returned by all BIFs to indicate an error, a value, or another condition.
#[allow(dead_code)]
pub enum BifResult {
  /// Totally legit result was returned.
  Value(LTerm),
  /// The bif has created an exception.
  Exception(ExceptionType, LTerm),
}


//impl From<fail::Error> for BifResult {
//  fn from(e: fail::Error) -> Self { BifResult::Fail(e) }
//}


impl fmt::Display for BifResult {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match *self {
      BifResult::Value(t) => write!(f, "Value({})", t),
      BifResult::Exception(et, rsn) => write!(f, "Exc({:?}, {})", et, rsn),
//      BifResult::Fail(ref e) => write!(f, "{:?}", e),
    }
  }
}
