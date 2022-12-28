use crate::{
  defs::{BitSize, ByteSize, WordSize},
  emulator::heap::{AllocInit, THeap},
  fail::RtResult,
  term::{
    boxed::{
      self,
      binary::trait_interface::TBinary,
      boxtype::{self, BoxType},
      trait_interface::TBoxed,
    },
    classify,
  },
};

/// Binary match buffer is a part of `BinaryMatchState`
struct MatchBuffer {
  // TODO: Make sure this is detected when garbage collected
  pub orig: *const dyn TBinary,
  /// The window begins at bit offset 0 always, and `start_at` will advance
  /// forward as we are reading from the binary.
  pub read_position: BitSize,
  pub stop_at: BitSize,
}

impl MatchBuffer {
  pub fn new(bin_ptr: *const dyn TBinary) -> Self {
    let stop_at = unsafe { (*bin_ptr).get_bit_size() };
    Self {
      orig: bin_ptr,
      read_position: BitSize::with_bits(0),
      stop_at,
    }
  }
}

/// Matchstate is stored on heap as a heap object. Followed by 1 or more save
/// offset `Term`s.
/// TODO: Merge match_buffer with this struct, because reasons?
pub struct BinaryMatchState {
  pub header: boxed::BoxHeader,
  match_buffer: MatchBuffer,
}

impl TBoxed for BinaryMatchState {
  fn get_class(&self) -> classify::TermClass {
    classify::CLASS_SPECIAL
  }

  fn get_type(&self) -> BoxType {
    boxtype::BOXTYPETAG_BINARY_MATCH_STATE
  }
}

impl BinaryMatchState {
  pub fn reset(&mut self) {
    println!("TODO: reset binary match state");
  }

  fn storage_size() -> WordSize {
    let bsize = ByteSize::new(std::mem::size_of::<Self>());
    bsize.get_words_rounded_up()
  }

  /// Create a new matchstate for the initial binary match step.
  fn new(bin_ptr: *const dyn TBinary) -> Self {
    let storage_size = Self::storage_size();
    let mut self_ = Self {
      header: boxed::BoxHeader::new::<BinaryMatchState>(storage_size),
      match_buffer: MatchBuffer::new(bin_ptr),
    };
    self_.reset();
    self_
  }

  pub unsafe fn create_into(
    bin_ptr: *const dyn TBinary,
    hp: &mut dyn THeap,
  ) -> RtResult<*mut BinaryMatchState> {
    let storage_sz = Self::storage_size();
    let this = hp.alloc(storage_sz, AllocInit::Uninitialized)? as *mut Self;

    // Create and write the block header (Self)
    let new_self = Self::new(bin_ptr);
    this.write(new_self);

    Ok(this)
  }

  #[inline]
  pub fn get_src_binary(&self) -> *const dyn TBinary {
    self.match_buffer.orig
  }

  #[inline]
  pub fn get_bits_remaining(&self) -> BitSize {
    let stop_at = self.match_buffer.stop_at.bits;
    let read_pos = self.match_buffer.read_position.bits;
    debug_assert!(
      read_pos <= stop_at,
      "Offset in match buffer {} can't and shouldn't be greater than the total bits {}",
      read_pos,
      stop_at
    );
    BitSize::with_bits(stop_at - read_pos)
  }

  #[inline]
  pub fn get_offset(&self) -> BitSize {
    self.match_buffer.read_position
  }

  pub fn increase_offset(&mut self, offs: BitSize) {
    self.match_buffer.read_position = self.match_buffer.read_position + offs;
  }
}
