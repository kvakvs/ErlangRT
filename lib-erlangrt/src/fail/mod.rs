//! Generic errors used everywhere in the ErlangRT runtime.
//!
pub mod create;

use crate::{
  beam::compact_term::CTError,
  defs::exc_type::ExceptionType,
  rt_util::bin_reader::{self, ReadError},
  term::lterm::LTerm,
};
use std::convert::From;

#[allow(dead_code)]
#[derive(Debug)]
pub enum RtErr {
  FileNotFound(String),
  ETFParseError(String),
  ReadError(ReadError),

  //--- Code loading ---
  CodeLoadingFailed(String),
  CodeLoadingCompactTerm(CTError),
  // PrematureEOF,

  //--- Code server, lookups ---
  NotFound, // generic notfound-anything
  ModuleNotFound(String),
  FunctionNotFound(String),
  BifNotFound(String),

  //--- Memory allocation events and errors ---
  AtomNotExist(String),
  // /// Very bad, no more memory to grow.
  // OutOfMemory,
  /// No space left in heap. GC requested.
  HeapIsFull,
  /// Attempt to index outside of the current stack.
  StackIndexRange(usize),

  //--- VM Checks --
  CreatingZeroSizedBinary, // can't create 0 sized bin on heap, use immediate {} instead
  Exception(ExceptionType, LTerm), // type, value
  TermIsNotABoxed,
  // used by `helper_get_mut_from_boxed_term` when boxed tag is different from
  // what is expected
  BoxedTagCheckFailed,
  BoxedIsNotABigint,
  BoxedIsNotAClosure,
  BoxedIsNotAnExport,
  BoxedIsNotAnImport,
  BoxedIsNotATuple,
  BoxedIsNotAMap,

  //--- Binary ---
  CannotCopyIntoRefbin, // To copy into binary, resolve ref into heapbin
  HeapBinTooSmall(usize, usize), // want bytes, have bytes
  ProcBinTooSmall(usize, usize), // want bytes, have bytes
}

impl From<bin_reader::ReadError> for RtErr {
  fn from(e: bin_reader::ReadError) -> Self {
    RtErr::ReadError(e)
  }
}

/// A templated error type based on `fail::Error`.
pub type RtResult<T> = Result<T, RtErr>;
