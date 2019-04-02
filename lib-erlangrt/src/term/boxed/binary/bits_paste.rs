//! Paste operations insert integers (small and big) somewhere into a binary.
//! Ported from OTP `erl_bits.c` mostly.

use core::ptr;

use crate::{
  beam::opcodes::binary::BsFlags,
  defs::{self, BitSize},
  fail::{RtErr, RtResult},
  term::{boxed::binary::trait_interface::TBinary, value::Term},
};
use crate::term::boxed;
use crate::term::boxed::bignum;
use crate::term::boxed::binary::bits;

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
      Self::copy_bits_from_offset_0_bytealigned_dst(
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
          return Self::put_bits_one_byte(
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
          let dst_ptr = dst.as_mut_ptr().add(dst_offset.get_bytes_rounded_down());
          return Self::paste_int_or_bigint(write_val, write_size, dst_ptr, flags);
        }
      } else if flags.contains(BsFlags::LITTLE) {
        unsafe {
          return Self::put_bits_unaligned();
        }
      } else {
        unsafe {
          let dst_ptr = dst.as_mut_ptr().add(dst_offset.get_bytes_rounded_down());
          return Self::put_bits_big_endian(
            write_val, write_size, rbits, dst_ptr, dst_offset, flags,
          );
        }
      }
    } else if dst_offset.is_empty() {
      // Big number, aligned on a byte boundary. We can format the
      // integer directly into the binary.
      unsafe {
        let dst_ptr = dst.as_mut_ptr().add(dst_offset.get_bytes_rounded_down());
        return Self::paste_int_or_bigint(write_val, write_size, dst_ptr, flags);
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
    iptr: *mut u8,
    _dst_offset: BitSize,
    flags: crate::beam::opcodes::BsFlags,
  ) -> RtResult<()> {
    // Big-endian, more than one byte, but not aligned on a byte boundary.
    // Handle the bits up to the next byte boundary specially,
    // then let fmt_int() handle the rest.
    let shift_count = write_size.bits - rbits;
    let val = write_val.get_small_signed();
    let mut b = ptr::read(iptr) & (0xff << (rbits & defs::BYTE_SHIFT_RANGE_MASK));

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
    ptr::write(iptr, b);

    // fmt_int() can't fail here. Continue to the next byte
    Self::paste_int_or_bigint(
      write_val,
      a,
      iptr.add(1),
      write_size - BitSize::with_bits(rbits),
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
    let mut b = ptr::read(iptr) & (0xff << (rbits & defs::BYTE_SHIFT_RANGE_MASK));
    let val_mask = (1 << write_size.bits) - 1;
    let new_val = (write_val & val_mask) << (8 - inbyte_offset - write_size.bits);
    b |= new_val as u8;
    ptr::write(iptr, b);
    Ok(())
  }

  unsafe fn put_bits_unaligned() -> RtResult<()> {
    unimplemented!("put_bits_unaligned");
    //    Ok(())
  }

  /// copy sz byte from val to dst buffer,
  /// dst, val are updated!!!
  #[inline]
  unsafe fn copy_val(mut dst: *mut u8, ddir: isize, mut val: isize, mut sz: usize) -> (*mut u8, isize) {
    while sz > 0 {
      ptr::write(dst, val as u8);
      dst = dst.offset(ddir);
      val >>= 8;
      sz -= 1;
    }
    (dst, val)
  }

  /// Writes bits of an integer respecting BIG/SMALL endian flags.
  /// Arg `val`: big or small integer.
  /// Assumes destination has enough space in it.
  /// Ported from OTP `fmt_int` in `erl_bits.c`
  unsafe fn paste_int_or_bigint(
    val: Term,
    size: BitSize,
    mut buf: *mut u8,
    buf_sz: usize,
    flags: BsFlags,
  ) -> RtResult<()> {
    if val.is_small() {
      paste_smallint(val, size, buf, buf_sz, flags)
    } else if val.is_big_int() {
      paste_bigint(val, size, buf, buf_sz, flags)
    } else {
      // Neither small nor big
      Err(RtErr::PasteIntMustBeSmallOrBigint);
    }
  }

  #[inline]
  unsafe fn paste_smallint(
    val: Term,
    size: BitSize,
    mut buf: *mut u8,
    buf_sz: usize,
    flags: BsFlags,
  ) -> RtResult<()> {
    let offs = size.get_last_byte_bits();
    let v = val.get_small_signed();

    debug_assert_ne!(size, 0); // Tested by the caller

    if flags.contains(BsFlags::LITTLE) {
      // if Little endian, copy from the beginning forward
      buf_sz -= 1;
      (buf, v) = copy_val(buf, 1, v, buf_sz);
      if offs != 0 {
        ptr::write(buf, (v << (defs::BYTE_BITS - offs) & defs::BYTE_SHIFT_RANGE_MASK) as u8);
      } else {
        ptr::write(buf, v as u8);
      }
    } else {
      // if Big endian, copy from the end back
      buf += buf_sz - 1;
      if offs != 0 {
        ptr::write(buf, (v << (defs::BYTE_BITS - offs)) as u8);
        buf = buf.offset(-1);
        buf_sz -= 1;
        v >>= offs;
      }
      (buf, v) = copy_val(buf, -1, v, buf_sz);
    }

    Ok(())
  }

  /// calculate a - *carry_ptr (carry)  (store result in b), *carry_ptr is updated!
  /// ported from `SUBc` macro in OTP `erl_bits.c`
  /// Takes carry current value, returns: new carry
  unsafe fn subc(x: u8, carry: usize, b: *mut u8) -> usize {
    let y = x - carry as u8;
    ptr::write(b, !y);
    if y > x { 1 } else { 0 }
  }

  #[inline]
  unsafe fn paste_bigint(
    val: Term,
    size: BitSize,
    mut buf: *mut u8,
    mut buf_sz: usize,
    flags: BsFlags,
  ) -> RtResult<()> {
    let mut offs = size.get_last_byte_bits();
    let big_ptr = val.get_box_ptr::<boxed::Bignum>();

    let sign = (*big_ptr).is_negative();
    let ds = (*big_ptr).get_byte_size();  // size of digit storage
    let digits = (*big_ptr).get_digits();
    let dp = digits.as_ptr();
    let n = core::cmp::min(buf_sz, ds.bytes());

    if size.bits == 0 {
      return Err(RtErr::PasteIntZeroDstSize);
    }

    if flags.contains(BsFlags::LITTLE) {
      buf_sz -= n; // pad with this amount

      if sign {
        buf = paste_bigint_le_negative(val, size, buf, buf_sz, flags, dp)
      } else {
        buf = paste_bigint_le_positive(val, size, buf, buf_sz, flags, dp)
      }
      // adjust MSB!!!
      if offs != 0 {
        buf = buf.offset(-1);
        ptr::write(buf, ptr::read(buf) << (defs::BYTE_BITS - offs));
      }
    } else {
      //
      // BIG ENDIAN
      //
      let mut acc: bignum::Digit = 0;

      buf = buf.add(buf_sz - 1); // end of buffer
      buf_sz -= n; // pad with this amount
      // shift offset
      offs = if offs != 0 { defs::BYTE_BITS - offs } else { 0 };

      if sign {
        // Big Endian & Negative
        let c = 1usize;

        while n >= bignum::BIG_DIGIT_SIZE {
          let d = ptr::read(dp);
          dp = dp.add(1);
          acc |= d << offs;
          SUBc((acc & 0xff), &c, buf);
          buf--;
          acc = d >> (8 - offs);
          for (i = 0; i < sizeof(ErtsDigit) - 1; ++i) {
            SUBc((acc & 0xff), &c, buf);
            buf--;
            acc >>= 8;
          }
          n -= sizeof(ErtsDigit);
        }
        if (n) {
          acc |= ((ErtsDigit) *dp << offs);
          do {
            SUBc((acc & 0xff), &c, buf);
            buf--;
            acc >>= 8;
          } while (--n > 0);
        }
        /* pad */
        while (buf_sz--) {
          SUBc((acc & 0xff), &c, buf);
          buf--;
          acc >>= 8;
        }
      } else { /* UNSIGNED */
        while (n >= sizeof(ErtsDigit)) {
          int i;

          d = *dp++;
          acc |= d << offs;
          *buf-- = acc;
          acc = d >> (8 - offs);
          for (i = 0; i < sizeof(ErtsDigit) - 1; ++i) {
            *buf-- = acc;
            acc >>= 8;
          }
          n -= sizeof(ErtsDigit);
        }
        if (n) {
          acc |= ((ErtsDigit) *dp << offs);
          do {
            *buf-- = acc & 0xff;
            acc >>= 8;
          } while (--n > 0);
        }
        while (buf_sz--) {
          *buf-- = acc & 0xff;
          acc >>= 8;
        }
      }
    }
    Ok(())
  }

/// Paste little endian bigint which is negative
/// Returns: Current write position
  #[inline]
  unsafe fn paste_bigint_le_negative(
    val: Term,
    size: BitSize,
    mut buf: *mut u8,
    mut buf_sz: usize,
    flags: BsFlags,
    mut dp: *const bignum::Digit,
  ) -> *mut u8 {
    let mut carry = 1usize;
    while n >= bignum::BIG_DIGIT_SIZE {
      let d = ptr::read(dp);
      dp = dp.add(1);
      for i in 0..bignum::BIG_DIGIT_SIZE) {
        carry = subc(d as u8, carry, buf);
        buf = buf.add(1);
        d >>= 8;
      }
      n -= bignum::BIG_DIGIT_SIZE;
    }
    if n != 0 {
      let d = ptr::read(dp);
      loop {
        carry = subc(d as u8, carry, buf);
        buf = buf.add(1);
        d >>= 8;
        n -= 1;
      } while n > 0;
    }
    // pad
    while buf_sz > 0 {
      buf_sz -= 1;
      carry = subc(0, carry, buf);
      buf = buf.add(1);
    }
    buf
  }

/// Paste little endian bigint which is positive.
/// Returns: Current write position
  #[inline]
  unsafe fn paste_bigint_le_positive(
    val: Term,
    size: BitSize,
    mut buf: *mut u8,
    mut buf_sz: usize,
    flags: BsFlags,
    mut dp: *const bignum::Digit,
  ) -> *mut u8 {
    while n >= bignum::BIG_DIGIT_SIZE {
      let d = ptr::read(dp);
      dp = dp.add(1);
      for i in 0..bignum::BIG_DIGIT_SIZE {
        ptr::write(buf, d as u8);
        buf = buf.add(1);
        d >>= 8;
      }
      n -= bignum::BIG_DIGIT_SIZE;
    }

    if n != 0 {
      let d = ptr::read(dp);
      loop {
        ptr::write(buf, d as u8);
        buf = buf.add(1);
        d >>= 8;
        n -= 1;
      } while n > 0;
    }
    // pad
    while buf_sz != 0 {
      ptr::write(buf, 0);
      buf = buf.add(1);
      buf_sz -= 1;
    }
    buf
  }
