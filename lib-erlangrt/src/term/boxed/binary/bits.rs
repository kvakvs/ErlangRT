use crate::{
  defs::{self, BitSize},
  fail::RtResult,
};

/// Generic bits copy algorithm which assumes non-byte-aligned `src` and `dst`.
/// The direction can be forward or backward.
/// Note: This is literal translation of OTP's erts_copy_bits in erl_bits.c
pub unsafe fn copy_bits(
  mut src: *const u8,
  mut src_offset: BitSize,
  src_direction: isize,
  mut dst: *mut u8,
  mut dst_offset: BitSize,
  dst_direction: isize,
  size: BitSize,
) -> RtResult<BitSize> {
  if size.bit_count == 0 {
    return Ok(size);
  }

  src = src.add(src_offset.get_byte_size_rounded_down().bytes());
  dst = dst.add(src_offset.get_byte_size_rounded_down().bytes());
  src_offset = BitSize::with_bits(src_offset.get_last_byte_bits());
  dst_offset = BitSize::with_bits(dst_offset.get_last_byte_bits());

  let dst_end_bits = (dst_offset + size).get_last_byte_bits();

  let mut lmask = if dst_offset.bit_count != 0 {
    make_mask(8 - dst_offset.bit_count)
  } else {
    0
  };

  let rmask = if dst_end_bits != 0 {
    make_mask(dst_end_bits) << (8 - dst_end_bits)
  } else {
    0
  };

  // Take care of the case that all bits are in the same byte.
  if (dst_offset + size).bit_count < defs::BYTE_BITS {
    // Check whether the masks overlap and should be `and`-ed, or should be `or`-ed
    lmask = if lmask & rmask != 0 {
      lmask & rmask
    } else {
      lmask | rmask
    };

    if src_offset == dst_offset {
      core::ptr::write(
        dst,
        mask_bits(core::ptr::read(src), core::ptr::read(dst), lmask as u8),
      );
    } else if src_offset.bit_count > dst_offset.bit_count {
      let diff_bits = (src_offset - dst_offset).get_last_byte_bits();
      let mut bits = ((core::ptr::read(src) as usize) << diff_bits) as u8;
      if (src_offset + size).bit_count > defs::BYTE_BITS {
        src = src.offset(src_direction);
        bits |= core::ptr::read(src) >> (defs::BYTE_BITS - diff_bits);
      }
      core::ptr::write(
        dst,
        mask_bits(bits as u8, core::ptr::read(dst), lmask as u8),
      );
    } else {
      let diff_bits = (src_offset - dst_offset).get_last_byte_bits();
      core::ptr::write(
        dst,
        mask_bits(
          core::ptr::read(src) >> diff_bits,
          core::ptr::read(dst),
          lmask as u8,
        ),
      );
    }
    return Ok(size); // We are done!
  }

  unimplemented!()
  // Ok(size)
}

/// Constructs a mask with n bits.
/// Example: make_mask(3) returns the binary number 00000111
#[inline]
fn make_mask(n: usize) -> usize {
  (1usize << n) - 1
}

/// Assign bits from `src` to same bits in `dst`, but preserve the `dst` bits
/// outside the mask.
#[inline]
fn mask_bits(src: u8, dst: u8, mask: u8) -> u8 {
  (src & mask) | (dst & !mask)
}
