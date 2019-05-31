use crate::{
  defs::*,
  term::{boxed::trait_interface::TBoxed, PrimaryTag},
};
use core::ptr;

/// Term header in memory, followed by corresponding data. The first header word
/// is parsed just like any term, tag bits are set to PrimaryTag::HEADER.
pub struct BoxHeader {
  /// Format is <arity> <TAG_HEADER:PrimaryTag::TAG_BITS>
  header_word: Word,

  /// Guard word is only present in debug build, is used to verify that the
  /// pointer to boxed value is not shifted or corrupted.
  #[cfg(debug_assertions)]
  guard_word: usize,

  /// Pointer to TBoxed trait vtable depending on the type. Necessary for
  /// reconstructing the fat trait pointer
  trait_vtab: *mut (),
}

/// In debug build, an extra word is added to each header on heap to assist
/// with debugging. NOTE: the last bits in ....cafe resolve to 4 (atom term).
#[cfg(debug_assertions)]
pub const GUARD_WORD_VALUE: usize = 0xfeebbeeffacecafe;

impl BoxHeader {
  pub fn new<TraitType>(storage_size: WordSize) -> BoxHeader
  where
    TraitType: TBoxed,
  {
    let arity = storage_size.words;
    let header_word = (arity << PrimaryTag::TAG_BITS) | PrimaryTag::HEADER.0;

    // Extract and store vtable pointer from the TBoxed trait object
    let trait_ptr = ptr::null_mut::<TraitType>() as *mut TBoxed;
    let trait_obj: core::raw::TraitObject = unsafe { core::mem::transmute(trait_ptr) };
    Self::create_bare(header_word, trait_obj.vtable)
  }

  #[cfg(debug_assertions)]
  pub const fn create_bare(header_word: usize, trait_vtab: *mut ()) -> Self {
    BoxHeader {
      header_word,
      guard_word: GUARD_WORD_VALUE,
      trait_vtab,
    }
  }

  #[cfg(not(debug_assertions))]
  pub const fn create_bare(header_word: usize, trait_vtab: *mut ()) -> Self {
    BoxHeader {
      header_word,
      trait_vtab,
    }
  }

  pub const fn storage_size() -> WordSize {
    // The size will include guard word on debug builds
    ByteSize::new(core::mem::size_of::<Self>()).get_words_rounded_up()
  }

  /// For any boxed header ensure that guard value is in place. Compiles to
  /// empty inline function in release.
  #[inline]
  #[cfg(debug_assertions)]
  pub fn ensure_valid(&self) {
    assert_eq!(
      self.guard_word, GUARD_WORD_VALUE,
      "Guard value 0x{:x} at {:p} is not in place (found value 0x{:x}), \
       check that the pointer to the box is aligned and correct",
      GUARD_WORD_VALUE, self as *const Self, self.guard_word
    );
  }

  #[cfg(not(debug_assertions))]
  #[inline]
  pub const fn ensure_valid(&self) {}

  #[inline]
  pub fn get_trait_ptr(&self) -> *const TBoxed {
    self.ensure_valid();
    let trait_obj = core::raw::TraitObject {
      data: self as *const Self as *mut (),
      vtable: self.trait_vtab,
    };
    unsafe { core::mem::transmute(trait_obj) }
  }

  #[inline]
  #[allow(dead_code)]
  pub fn get_trait_ptr_mut(&mut self) -> *mut TBoxed {
    self.ensure_valid();
    let trait_obj = core::raw::TraitObject {
      data: self as *mut Self as *mut (),
      vtable: self.trait_vtab,
    };
    unsafe { core::mem::transmute(trait_obj) }
  }

  pub fn get_storage_size(&self) -> WordSize {
    self.ensure_valid();
    Self::headerword_to_storage_size(self.header_word)
  }

  /// For a header word value, extract bits with arity
  /// Format is <arity> <boxtype:BOXTYPE_TAG_BITS> <TAG_HEADER:PrimaryTag::TAG_BITS>
  #[inline]
  pub fn headerword_to_storage_size(w: Word) -> WordSize {
    debug_assert_eq!(
      w & PrimaryTag::TAG_MASK,
      PrimaryTag::HEADER.0,
      "Boxed header with arity must have HEADER tag"
    );
    WordSize::new(w >> PrimaryTag::TAG_BITS)
  }
}
