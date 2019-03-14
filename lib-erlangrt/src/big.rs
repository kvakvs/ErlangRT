use crate::{
  defs,
  emulator::heap::Heap,
  fail::RtResult,
  term::{
    boxed::{self, endianness::Endianness},
    value::Term,
  },
};

/// Helper, creates a new big integer on heap with val.
pub fn from_isize(hp: &mut Heap, val: isize) -> RtResult<Term> {
  let p = boxed::Bignum::with_isize(hp, val)?;
  Ok(Term::make_boxed(p))
}

/// Multiply two big ints
pub fn mul(_hp: &mut Heap, a: Term, b: Term) -> RtResult<Term> {
  debug_assert!(a.is_big_int());
  debug_assert!(b.is_big_int());
  unimplemented!("mul big a, b")
}

#[allow(dead_code)]
pub fn isize_from(val: Term) -> Option<isize> {
  if val.is_small() {
    return Some(val.get_small_signed());
  }
  unimplemented!("isize from big")
}

/// From array of bytes create limb array for bignum.
/// Least significant limb goes first.
#[cfg(target_endian = "little")]
pub fn make_limbs_from_bytes(input_endian: Endianness, b: Vec<u8>) -> Vec<usize> {
  // Only proceed if input is same endianness as the machine endianness
  assert_eq!(input_endian, Endianness::default());

  // Round dst size up to nearest word size which will fit everything
  let dst_size = (b.len() + defs::WORD_BYTES - 1) / defs::WORD_BYTES;
  let mut dst = Vec::<usize>::with_capacity(dst_size);
  unsafe {
    core::ptr::copy_nonoverlapping(
      b.as_ptr(),
      dst.as_mut_ptr() as *mut u8,
      b.len() & !(defs::WORD_BITS - 1),
    );
    // Bytes which did not form a new full usize
    let remaining_bytes = b.len() & (defs::WORD_BITS - 1);
    if remaining_bytes > 0 && dst.len() > 0 {
      let index = dst.len() - 1;
      dst[index] = 0;
      core::ptr::copy_nonoverlapping(
        b.as_ptr().add(index * defs::WORD_BYTES),
        dst.as_mut_ptr().add(index) as *mut u8,
        remaining_bytes,
      );
    }
  }
  dst
}
