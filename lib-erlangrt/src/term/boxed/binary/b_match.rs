use crate::{
  defs::{ByteSize, WordSize},
  emulator::heap::Heap,
  fail::RtResult,
  term::boxed::{self, binary::trait_interface::TBinary},
};

/// Binary match buffer is a part of `BinaryMatchState`
struct MatchBuffer {
  // TODO: Make sure this is detected when garbage collected
  pub orig: *const TBinary,
  pub base: *const u8,
  pub offset: usize,
  pub bit_size: usize,
}

impl MatchBuffer {
  pub fn new(bin_ptr: *const TBinary) -> Self {
    Self {
      orig: bin_ptr,
      base: unsafe { (*bin_ptr).get_data() },
      offset: 0,
      bit_size: 0,
    }
  }
}

/// Matchstate is stored on heap as a heap object. Followed by 1 or more save
/// offset `LTerm`s.
pub struct BinaryMatchState {
  pub header: boxed::BoxHeader,
  match_buffer: MatchBuffer,
}

impl BinaryMatchState {
  pub fn reset(&mut self) {
    println!("TODO: reset binary match state");
  }

  fn storage_size() -> WordSize {
    let bsize = ByteSize::new(std::mem::size_of::<Self>());
    bsize.words_rounded_up()
  }

  /// Create a new matchstate for the initial binary match step.
  fn new(bin_ptr: *const TBinary) -> Self {
    let arity = Self::storage_size();
    let mut self_ = Self {
      header: boxed::BoxHeader::new(boxed::BOXTYPETAG_BINARY_MATCH_STATE, arity.words()),
      match_buffer: MatchBuffer::new(bin_ptr),
    };
    self_.reset();
    self_
  }

  pub unsafe fn create_into(
    bin_ptr: *const TBinary,
    hp: &mut Heap,
  ) -> RtResult<*mut BinaryMatchState> {
    let storage_sz = Self::storage_size();
    let this = hp.alloc::<Self>(storage_sz, false)?;

    // Create and write the block header (Self)
    let new_self = Self::new(bin_ptr);
    core::ptr::write(this, new_self);

    Ok(this)
  }

  #[inline]
  pub fn get_src_binary(&self) -> *const TBinary {
    self.match_buffer.orig
  }
}
