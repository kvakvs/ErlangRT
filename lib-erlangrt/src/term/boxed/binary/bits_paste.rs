//! Paste operations insert integers (small and big) somewhere into a binary.
//! Ported from OTP `erl_bits.c` mostly.

use core::{cmp, ptr};

use crate::{
  beam::opcodes::binary::BsFlags,
  defs::{self, BitSize},
  fail::{RtErr, RtResult},
  term::{
    boxed::{
      self, bignum,
      binary::{bits, trait_interface::TBinary},
    },
    value::Term,
  },
};

pub enum SizeOrAll {
  Bits(BitSize),
  All,
}

/// Writes binary `src` into binary `dst`, with a bit offset.
/// Arg `src`: TBinary serving as data source,
/// Arg `dst_offset`: Bit offset in the destination where the bits go
/// Arg `size`: How many bits to copy, or `All`
/// Returns: Bit count copied
pub unsafe fn put_binary(
  src: *const TBinary,
  size_or_all: SizeOrAll,
  dst: *mut TBinary,
  dst_offset: BitSize,
  _flags: crate::beam::opcodes::BsFlags,
) -> RtResult<BitSize> {
  let size = match size_or_all {
    SizeOrAll::All => (*src).get_bit_size(),
    SizeOrAll::Bits(s) => s,
  };

  let dst_size = (*dst).get_bit_size();
  if dst_offset + size > dst_size {
    return Err(RtErr::BinaryDestinationTooSmall);
  }

  // Start reading from offset 0
  let src_data = (*src).get_data();
  if dst_offset.get_last_byte_bits() == 0 {
    copy_bits_from_offset_0_bytealigned_dst(
      src_data,
      size,
      (*dst).get_data_mut(),
      dst_offset,
    )
  } else {
    bits::copy_bits(
      src_data.as_ptr(),
      BitSize::zero(),
      1,
      (*dst).get_data_mut().as_mut_ptr(),
      dst_offset,
      1,
      size,
    )
  }
}

/// Copies `size` bits from the beginning of `src` slice into `dst` slice
/// at `dst_offset` bits, where `dst_offset` must be byte aligned.
fn copy_bits_from_offset_0_bytealigned_dst(
  src: &[u8],
  size: BitSize,
  dst: &mut [u8],
  dst_offset: BitSize,
) -> RtResult<BitSize> {
  unsafe {
    let dst_offset_bytes = dst_offset.get_byte_size_rounded_down().bytes();
    let size_bytes = size.get_byte_size_rounded_down().bytes();
    ptr::copy_nonoverlapping(
      src.as_ptr(),
      dst.as_mut_ptr().add(dst_offset_bytes),
      size_bytes,
    );
  }

  // Possibly there is an incomplete byte at the tail, to be copied
  let remaining_bits = size.get_last_byte_bits();
  if remaining_bits != 0 {
    unimplemented!("Copy partial byte")
  }

  // Pass the size out to the caller as they might have called
  // `BinaryWriter::put_binary` with just a `SizeOrAll::All` value and might
  // be interested in the actual size copied
  Ok(size)
}

/// For a writable byte buffer, insert an integer of given size. Different cases
/// are handled for offsets multiple of 8, and for small/big integers.
pub fn put_integer(
  write_val: Term,
  write_size: BitSize,
  dst: &mut [u8],
  dst_offset: BitSize,
  flags: crate::beam::opcodes::BsFlags,
) -> RtResult<()> {
  if write_size.is_empty() {
    // Nothing to do
    return Ok(());
  }

  if write_val.is_small() {
    let inbyte_offset = dst_offset.get_last_byte_bits();
    let rbits = defs::BYTE_BITS - inbyte_offset;

    if inbyte_offset + write_size.bits < defs::BYTE_BITS {
      // All bits will land into the same byte
      unsafe {
        let iptr = dst.as_mut_ptr().add(dst_offset.get_bytes_rounded_down());
        return put_bits_one_byte(
          iptr,
          rbits,
          inbyte_offset,
          write_val.get_small_signed(),
          write_size,
        );
      }
    } else if inbyte_offset == 0 {
      // More than one bit, starting at a byte boundary.
      unsafe {
        return fmt_int(
          write_val,
          write_size,
          dst,
          dst_offset.get_bytes_rounded_down(),
          flags,
        );
      }
    } else if flags.contains(BsFlags::LITTLE) {
      unsafe {
        return put_bits_unaligned();
      }
    } else {
      unsafe {
        return put_bits_big_endian(
          write_val,
          write_size,
          rbits,
          dst,
          dst_offset.get_bytes_rounded_down(),
          flags,
        );
      }
    }
  } else if dst_offset.is_empty() {
    // Big number, aligned on a byte boundary. We can format the
    // integer directly into the binary.
    unsafe {
      // Dst.len here might need to be adjusted to smaller length with dst bit offset
      return fmt_int(
        write_val,
        write_size,
        dst,
        dst_offset.get_bytes_rounded_down(),
        flags,
      );
    }
  } else {
    // unaligned
    unimplemented!("put_integer: unaligned")
  }

  // Ok(())
}

unsafe fn put_bits_big_endian(
  write_val: Term,
  write_size: BitSize,
  rbits: usize,
  dst: &mut [u8],
  dst_offset: usize,
  flags: crate::beam::opcodes::BsFlags,
) -> RtResult<()> {
  // Big-endian, more than one byte, but not aligned on a byte boundary.
  // Handle the bits up to the next byte boundary specially,
  // then let fmt_int() handle the rest.
  let shift_count = write_size.bits - rbits;
  let val = write_val.get_small_signed();
  let mut b = dst[dst_offset] & (0xff << defs::byte_shift(rbits));

  // Shifting with a shift count greater than or equal to the word
  // size may be a no-op (instead of 0 the result may be the unshifted
  // value). Therefore, only do the shift and the OR if the shift count
  // is less than the word size if the number is positive; if negative,
  // we must simulate the sign extension.

  if shift_count < defs::WORD_BITS {
    let add_bits = (val >> shift_count) & ((1 << rbits) - 1);
    b |= add_bits as u8;
  } else if val < 0 {
    // Simulate sign extension.
    b |= (!0) & ((1 << rbits) - 1);
  }
  dst[dst_offset] = b;

  // fmt_int() can't fail here. Continue to the next byte
  fmt_int(
    write_val,
    write_size - BitSize::with_bits(rbits),
    dst,
    dst_offset + 1,
    flags,
  )
}

/// Destination span is entirely inside one byte
#[inline]
unsafe fn put_bits_one_byte(
  iptr: *mut u8,
  rbits: usize,
  inbyte_offset: usize,
  write_val: isize,
  write_size: BitSize,
) -> RtResult<()> {
  // Read the old value and mask away the bits about to be replaced
  let mut b = iptr.read() & (0xff << defs::byte_shift(rbits));
  let val_mask = (1 << write_size.bits) - 1;
  let new_val = (write_val & val_mask) << (8 - inbyte_offset - write_size.bits);
  b |= new_val as u8;
  iptr.write(b);
  Ok(())
}

unsafe fn put_bits_unaligned() -> RtResult<()> {
  unimplemented!("put_bits_unaligned");
}

/// copy sz byte from val to dst buffer,
/// dst, val are updated!!!
/// Returns: new offset updated by adding direction to the old offset, and the value
#[inline]
fn copy_and_update_val(
  dst: &mut [u8],
  mut dst_offset: usize,
  ddir: isize,
  mut val: isize,
) -> (usize, isize) {
  let mut sz = dst.len() - dst_offset;
  while sz > 0 {
    dst[dst_offset] = val as u8;
    dst_offset = (dst_offset as isize + ddir) as usize;
    val >>= 8;
    sz -= 1;
  }
  (dst_offset, val)
}

/// Writes bits of an integer respecting BIG/SMALL endian flags.
/// Assumes destination has enough space in it.
/// Arg `val`: big or small integer.
/// Ported from OTP `fmt_int` in `erl_bits.c`
unsafe fn fmt_int(
  write_val: Term,
  write_size: BitSize,
  dst: &mut [u8],
  dst_offset: usize,
  flags: BsFlags,
) -> RtResult<()> {
  if write_val.is_small() {
    paste_smallint(write_val, write_size, dst, dst_offset, flags)
  } else if write_val.is_big_int() {
    paste_bigint(write_val, write_size, dst, dst_offset, flags)
  } else {
    // Neither small nor big
    Err(RtErr::PasteIntMustBeSmallOrBigint)
  }
}

#[inline]
unsafe fn paste_smallint(
  val: Term,
  size: BitSize,
  dst: &mut [u8],
  mut dst_offset: usize,
  flags: BsFlags,
) -> RtResult<()> {
  let offs = size.get_last_byte_bits();
  let mut v = val.get_small_signed();

  debug_assert!(!size.is_empty()); // Tested by the caller

  if flags.contains(BsFlags::LITTLE) {
    // if Little endian, copy from the beginning forward
    let tmp1 = copy_and_update_val(dst, dst_offset, 1, v);
    dst_offset = tmp1.0;
    v = tmp1.1;

    if offs != 0 {
      dst[dst_offset] = (v << defs::byte_shift(defs::BYTE_BITS - offs)) as u8;
    } else {
      dst[dst_offset] = v as u8;
    }
  } else {
    // if Big endian, copy from the end back
    dst_offset = dst.len() - 1;
    if offs != 0 {
      dst[dst_offset] = (v << (defs::BYTE_BITS - offs)) as u8;
      dst_offset -= 1;
      v >>= offs;
    }
    copy_and_update_val(dst, dst_offset, -1, v);
    // dst_offset = tmp2.0;
    // v = tmp2.1;
  }

  Ok(())
}

/// calculate a - carry (store result in `dst[dstoffset]`)
/// Returns: Updated carry
/// ported from `SUBc` macro in OTP `erl_bits.c`
unsafe fn subc(x: u8, carry: usize, dst: &mut [u8], dst_offset: usize) -> usize {
  let y = x - carry as u8;
  dst[dst_offset] = !y;
  if y > x {
    1
  } else {
    0
  }
}

/// Insert a value `val` which is possibly a bigint, into a window
/// of `size` in byte buffer at `buf`, with limit `buf_sz`
#[inline]
unsafe fn paste_bigint(
  val: Term,
  size: BitSize,
  dst: &mut [u8],
  mut dst_offset: usize,
  flags: BsFlags,
) -> RtResult<()> {
  let mut offs = size.get_last_byte_bits();
  let big_ptr = val.get_box_ptr::<boxed::Bignum>();

  let sign = (*big_ptr).is_negative();
  let _ds = (*big_ptr).get_byte_size(); // size of digit storage
  let digits = (*big_ptr).get_digits();
  let dp = digits.as_ptr();

  if size.bits == 0 {
    return Err(RtErr::PasteIntZeroDstSize);
  }

  if flags.contains(BsFlags::LITTLE) {
    // buf_sz -= n; // pad with this amount

    if sign {
      dst_offset = paste_bigint_le_negative(val, size, dst, dst_offset, flags, dp)
    } else {
      dst_offset = paste_bigint_le_positive(val, size, dst, dst_offset, flags, dp)
    }
    // adjust MSB!!!
    if offs != 0 {
      dst_offset -= 1;
      dst[dst_offset] = dst[dst_offset] << (defs::BYTE_BITS - offs);
    }
  } else {
    // BIG ENDIAN
    //
    // let acc: bignum::Digit = 0;

    // dst_offset = dst.len() - 1; // end of buffer

    // shift offset
    offs = if offs != 0 { defs::BYTE_BITS - offs } else { 0 };

    if sign {
      paste_bigint_be_negative(val, size, dst, flags, dp, offs);
    } else {
      paste_bigint_be_positive(val, size, dst, flags, dp, offs);
    }
  }
  Ok(())
}

/// Paste little endian bigint which is negative
/// Returns: Updated dst offset
#[inline]
unsafe fn paste_bigint_le_negative(
  _val: Term,
  _size: BitSize,
  dst: &mut [u8],
  mut dst_offset: usize,
  _flags: BsFlags,
  mut dp: *const bignum::Digit,
) -> usize {
  let mut carry = 1usize;
  let mut n = cmp::min(dst.len(), bignum::BIG_DIGIT_SIZE);
  while n >= bignum::BIG_DIGIT_SIZE {
    let mut d = dp.read();
    dp = dp.add(1);
    for _i in 0..bignum::BIG_DIGIT_SIZE {
      carry = subc(d as u8, carry, dst, dst_offset);
      dst_offset += 1;
      d >>= 8;
    }
    n -= bignum::BIG_DIGIT_SIZE;
  }
  if n != 0 {
    let mut d = dp.read();
    loop {
      carry = subc(d as u8, carry, dst, dst_offset);
      dst_offset += 1;
      d >>= 8;
      n -= 1;
      if n <= 0 {
        break;
      }
    }
  }
  // pad
  while dst_offset < dst.len() {
    carry = subc(0, carry, dst, dst_offset);
    dst_offset += 1;
  }
  dst_offset
}

/// Paste little endian bigint which is positive.
/// Returns: Current write position
#[inline]
unsafe fn paste_bigint_le_positive(
  _val: Term,
  _size: BitSize,
  dst: &mut [u8],
  mut dst_offset: usize,
  _flags: BsFlags,
  mut dp: *const bignum::Digit,
) -> usize {
  let mut n = cmp::min(dst.len(), bignum::BIG_DIGIT_SIZE);
  while n >= bignum::BIG_DIGIT_SIZE {
    let mut d = dp.read();
    dp = dp.add(1);
    for _i in 0..bignum::BIG_DIGIT_SIZE {
      dst[dst_offset] = d as u8;
      dst_offset += 1;
      d >>= 8;
    }
    n -= bignum::BIG_DIGIT_SIZE;
  }

  if n != 0 {
    let mut d = dp.read();
    loop {
      dst[dst_offset] = d as u8;
      dst_offset += 1;
      d >>= 8;
      n -= 1;
      if n <= 0 {
        break;
      }
    }
  }
  // pad
  while dst_offset != 0 {
    dst[dst_offset] = 0;
    dst_offset -= 1;
  }
  dst_offset
}

#[inline]
unsafe fn paste_bigint_be_negative(
  _val: Term,
  _size: BitSize,
  dst: &mut [u8],
  _flags: BsFlags,
  mut dp: *const bignum::Digit,
  offs: usize,
) -> usize {
  let _dst_offset: usize = 0;
  let mut n = cmp::min(dst.len(), bignum::BIG_DIGIT_SIZE);
  // Big Endian & Negative
  let mut dst_offset: usize = dst.len() - 1;
  let mut carry = 1usize;
  let mut acc: bignum::Digit = 0;

  while n >= bignum::BIG_DIGIT_SIZE {
    let d = dp.read();
    dp = dp.add(1);
    acc |= d << offs;
    carry = subc(acc as u8, carry, dst, dst_offset);
    dst_offset -= 1;
    acc = d >> (8 - offs);
    for _i in 0..bignum::BIG_DIGIT_SIZE {
      carry = subc(acc as u8, carry, dst, dst_offset);
      dst_offset -= 1;
      acc >>= 8;
    }
    n -= bignum::BIG_DIGIT_SIZE;
  }
  if n != 0 {
    acc |= dp.read() << offs;
    loop {
      carry = subc(acc as u8, carry, dst, dst_offset);
      dst_offset -= 1;
      acc >>= 8;
      n -= 1;
      if n <= 0 {
        break;
      }
    }
  }
  // pad
  while dst_offset > 0 {
    dst_offset -= 1;
    carry = subc(acc as u8, carry, dst, dst_offset);
    dst_offset -= 1;
    acc >>= 8;
  }
  dst_offset
}

#[inline]
unsafe fn paste_bigint_be_positive(
  _val: Term,
  _size: BitSize,
  dst: &mut [u8],
  _flags: BsFlags,
  mut dp: *const bignum::Digit,
  offs: usize,
) -> usize {
  let mut n = cmp::min(dst.len(), bignum::BIG_DIGIT_SIZE);
  let mut dst_offset: usize = dst.len() - 1;
  let mut acc: bignum::Digit = 0;

  while n >= bignum::BIG_DIGIT_SIZE {
    let d = dp.read();
    dp = dp.add(1);
    acc |= d << defs::byte_shift(offs);
    dst_offset -= 1;
    dst[dst_offset] = acc as u8;
    acc = d >> defs::byte_shift(8 - offs);
    for _i in 0..bignum::BIG_DIGIT_SIZE {
      dst_offset -= 1;
      dst[dst_offset] = acc as u8;
      acc >>= 8;
    }
    n -= bignum::BIG_DIGIT_SIZE;
  }
  if n != 0 {
    acc |= dp.read() << offs;
    loop {
      dst_offset -= 1;
      dst[dst_offset] = acc as u8;
      acc >>= 8;
      n -= 1;
      if n <= 0 {
        break;
      }
    }
  }
  while dst_offset > 0 {
    dst_offset -= 1;
    dst[dst_offset] = acc as u8;
    acc >>= 8;
  }
  dst_offset
}
