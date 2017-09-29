use std::fmt;

pub enum Error {
  FileNotFound(String),
}

impl fmt::Debug for Error {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      &Error::FileNotFound(ref filename) =>
        return write!(f, "File not found: {}", filename)
    }
    write!(f, "Some internal error")
  }
}