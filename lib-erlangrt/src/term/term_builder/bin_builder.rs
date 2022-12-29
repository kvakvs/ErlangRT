use crate::{
  defs::sizes::SizeBytes,
  emulator::heap::THeap,
  fail::RtResult,
  term::{
    boxed::{self, binary::TBinary},
    Term,
  },
};

pub struct BinaryBuilder {
  p: *mut dyn TBinary,
  write_pos: *mut u8,
  limit: *mut u8,
  size: SizeBytes, // used in debug only
}

impl BinaryBuilder {
  #[inline]
  pub fn with_size(size: SizeBytes, hp: &mut dyn THeap) -> RtResult<Self> {
    let p = unsafe { boxed::Binary::create_into(size.get_bits(), hp) }?;
    let write_slice = unsafe { (*p).get_data_mut() };
    let write_pos = write_slice.as_mut_ptr();

    Ok(Self {
      p,
      write_pos,
      limit: unsafe { write_pos.add(size.bytes()) },
      size,
    })
  }

  pub unsafe fn write_byte(&mut self, b: u8) {
    debug_assert!(
      self.write_pos < self.limit,
      "binary_builder: writing beyond {} bytes",
      self.size
    );
    self.write_pos.write(b);
    self.write_pos = self.write_pos.add(1);
  }

  pub fn make_term(self) -> Term {
    unsafe { (*self.p).make_term() }
  }
}
