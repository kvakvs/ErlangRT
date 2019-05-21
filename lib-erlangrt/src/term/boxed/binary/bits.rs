use crate::{
  defs::{self, BitSize},
  fail::RtResult,
};
use core::ptr;

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
  if size.bits == 0 {
    return Ok(size);
  }

  src = src.add(src_offset.get_byte_size_rounded_down().bytes());
  dst = dst.add(src_offset.get_byte_size_rounded_down().bytes());
  src_offset = BitSize::with_bits(src_offset.get_last_byte_bits());
  dst_offset = BitSize::with_bits(dst_offset.get_last_byte_bits());

  let dst_end_bits = (dst_offset + size).get_last_byte_bits();

  let mut lmask = if dst_offset.bits != 0 {
    make_mask(defs::BYTE_BITS - dst_offset.bits)
  } else {
    0
  };

  let rmask = if dst_end_bits != 0 {
    make_mask(dst_end_bits) << (defs::BYTE_BITS - dst_end_bits)
  } else {
    0
  };

  // Take care of the case that all bits are in the same byte.
  if (dst_offset + size).bits < defs::BYTE_BITS {
    // Check whether the masks overlap and should be `and`-ed, or should be `or`-ed
    lmask = if lmask & rmask != 0 {
      lmask & rmask
    } else {
      lmask | rmask
    };

    if src_offset == dst_offset {
      dst.write(mask_bits(src.read(), dst.read(), lmask as u8));
    } else if src_offset.bits > dst_offset.bits {
      let diff_bits = (src_offset - dst_offset).get_last_byte_bits();
      let mut bits = ((src.read() as usize) << diff_bits) as u8;
      if (src_offset + size).bits > defs::BYTE_BITS {
        src = src.offset(src_direction);
        bits |= src.read() >> (defs::BYTE_BITS - diff_bits);
      }
      dst.write(mask_bits(bits as u8, dst.read(), lmask as u8));
    } else {
      let diff_bits = (src_offset - dst_offset).get_last_byte_bits();
      dst.write(mask_bits(
        src.read() >> diff_bits,
        ptr::read(dst),
        lmask as u8,
      ));
    }
    return Ok(size); // We are done!
  }

  // At this point, we know that the bits are in 2 or more bytes
  //
  let mut count = if lmask != 0 {
    (size.bits - (defs::BYTE_BITS - dst_offset.bits)) >> defs::BYTE_POF2_BITS
  } else {
    size.bits >> defs::BYTE_POF2_BITS
  };

  if src_offset == dst_offset {
    // The bits are aligned in the same way. We can just copy the bytes
    // (except for the first and last bytes). Note that the directions
    // might be different, so we can't just use memcpy().

    if lmask != 0 {
      dst.write(mask_bits(src.read(), dst.read(), lmask as u8));
      dst = dst.offset(dst_direction);
      src = src.offset(src_direction);
    }

    while count > 0 {
      count -= 1;
      dst.write(ptr::read(src));
      dst = dst.offset(dst_direction);
      src = src.offset(src_direction);
    }

    if rmask != 0 {
      dst.write(mask_bits(src.read(), dst.read(), rmask as u8));
    }
  } else {
    // The tricky case. The bits must be shifted into position.
    let lshift: usize;
    let rshift: usize;
    let mut bits: u8;

    if src_offset > dst_offset {
      lshift = (src_offset - dst_offset).bits;
      rshift = defs::byte_shift(defs::BYTE_BITS - lshift);
      bits = ptr::read(src);
      if (src_offset + size).bits > 8 {
        src = src.offset(src_direction);
      }
    } else {
      rshift = defs::byte_shift((dst_offset - src_offset).bits);
      lshift = defs::byte_shift(defs::BYTE_BITS - rshift);
      bits = 0;
    }

    if lmask != 0 {
      let mut bits1 = bits << lshift;
      bits = src.read();
      src = src.offset(src_direction);
      bits1 |= bits >> rshift;
      dst.write(mask_bits(bits1, dst.read(), lmask as u8));
      dst = dst.offset(dst_direction);
    }

    while count > 0 {
      count -= 1;
      let bits1 = bits << lshift;
      bits = src.read();
      src = src.offset(src_direction);
      dst.write(bits1 | (bits >> rshift));
      dst = dst.offset(dst_direction);
    }

    if rmask != 0 {
      let mut bits1 = bits << lshift;
      if (rmask << rshift) & 0xff != 0 {
        bits = src.read();
        bits1 |= bits >> rshift;
      }
      dst.write(mask_bits(bits1, dst.read(), rmask as u8));
    }
  }

  Ok(size)
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
