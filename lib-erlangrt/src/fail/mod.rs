//! Generic errors used everywhere in the ErlangRT runtime.
pub mod create;

use crate::{
  beam::loader::CompactTermError,
  defs::{exc_type::ExceptionType, ByteSize},
  rt_util::bin_reader::{self, ReadError},
  term::Term,
};
use std::convert::From;

#[allow(dead_code)]
#[derive(Debug)]
pub enum RtErr {
  BEAMFileNotFound(String),
  ETFParseError(String),
  ReadError(ReadError),

  //--- Code loading ---
  CodeLoadingFailed(String),
  CodeLoadingCompactTerm(CompactTermError),

  //--- Code server, lookups ---
  NotFound, // generic notfound-anything
  ModuleNotFound(String),
  FunctionNotFound(String),
  BifNotFound(String),

  //--- Memory allocation events and errors ---
  AtomNotExist(String),
  /// No space left in heap. GC requested.
  HeapIsFull,
  /// Attempt to index outside of the current stack.
  StackIndexRange(usize),

  //--- VM Checks --
  Exception(ExceptionType, Term), // type, value
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
  CreatingZeroSizedBinary, // can't create 0-sized bin on heap, use immediate {} instead
  CreatingZeroSizedSlice,  // can't create 0-sized slice, use immediate {} instead
  CannotCopyIntoRefbin,    // To copy into binary, resolve ref into heapbin
  CannotCopyIntoBinSlice,  // Can not copy into binary slice, it is const
  HeapBinTooSmall(usize, ByteSize), // want bytes, have bytes
  ProcBinTooSmall(usize, ByteSize), // want bytes, have bytes
  BinaryDestinationTooSmall, // bytes/bits will not fit the dst binary
  PasteIntMustBeSmallOrBigint,
  PasteIntZeroDstSize, // destination size for paste int was 0
}

impl From<bin_reader::ReadError> for RtErr {
  fn from(e: bin_reader::ReadError) -> Self {
    RtErr::ReadError(e)
  }
}

/// A templated error type based on `fail::Error`.
pub type RtResult<T> = Result<T, RtErr>;
