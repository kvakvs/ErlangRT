//! Generic binary-write bit-aware functions

use crate::{defs::BitSize, fail::RtResult, term::lterm::Term};

/// For a writable byte buffer, insert an integer of given size. Different cases
/// are handled for offsets multiple of 8, and for small/big integers.
pub fn put_integer(src: Term, size: BitSize, dst: &mut [u8], offset: BitSize) -> RtResult<()> {
  unimplemented!("boxed::binary::writing::put_integer")
}
