//!
//! Generic errors used everywhere in the VM runtime.
//!
use crate::beam::compact_term::CTError;
use crate::emulator::heap::HeapError;
use crate::rt_defs::ExceptionType;
use crate::rt_util::bin_reader;
use crate::rt_util::bin_reader::ReadError;
use crate::term::lterm::LTerm;
use std::convert::From;


// TODO: Rename to RTError-something
#[derive(Debug)]
pub enum Error {
  FileNotFound(String),
  ETFParseError(String),
  ReadError(ReadError),

  //--- Code loading ---
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
  BoxedIsNotAnExport,
  BoxedIsNotATuple,

  //--- Binary ---
  CannotCopyIntoRefbin,
  HeapBinTooSmall(usize, usize), // want bytes, have bytes
  ProcBinTooSmall(usize, usize), // want bytes, have bytes
}


impl From<bin_reader::ReadError> for Error {
  fn from(e: bin_reader::ReadError) -> Self {
    Error::ReadError(e)
  }
}

impl From<HeapError> for Error {
  fn from(e: HeapError) -> Self {
    Error::HeapError(e)
  }
}


/// A templated error type based on `fail::Error`.
pub type RtResult<T> = Result<T, Error>;
