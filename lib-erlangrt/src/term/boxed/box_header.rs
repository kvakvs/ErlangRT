use crate::{
  defs::*,
  term::{
    boxed::trait_interface::TBoxed,
    value::{PrimaryTag, TERM_TAG_BITS, TERM_TAG_MASK},
  },
};

/// Term header in memory, followed by corresponding data. The first header word
/// is parsed just like any term, tag bits are set to TermTag::HEADER.
pub struct BoxHeader {
  /// Format is <arity> <TAG_HEADER:TERM_TAG_BITS>
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
/// with debugging. NOTE: the last bits in ....2222 resolve to a CONS type term.
#[cfg(debug_assertions)]
pub const GUARD_WORD_VALUE: usize = 0x1111beefface2222;

impl BoxHeader {
  pub fn new<TraitType>(storage_size: WordSize) -> BoxHeader
  where
    TraitType: TBoxed,
  {
    let arity = storage_size.words();
    let header_word = (arity << TERM_TAG_BITS) | PrimaryTag::HEADER.get();

    // Extract and store vtable pointer from the TBoxed trait object
    let trait_ptr = core::ptr::null_mut::<TraitType>() as *mut TBoxed;
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
    ByteSize::new(core::mem::size_of::<Self>()).get_words_rounded_up()
  }

  /// For any boxed header ensure that guard value is in place. Compiles to
  /// empty inline function in release.
  #[inline]
  #[cfg(debug_assertions)]
  pub fn ensure_valid(&self) {
    assert_eq!(
      self.guard_word, GUARD_WORD_VALUE,
      "Guard value is not in place, check that the pointer to the box is correct"
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

  pub fn get_arity(&self) -> usize {
    self.ensure_valid();
    headerword_to_arity(self.header_word)
  }
}

/// For a header word value, extract bits with arity
/// Format is <arity> <boxtype:BOXTYPE_TAG_BITS> <TAG_HEADER:TERM_TAG_BITS>
pub fn headerword_to_arity(w: Word) -> usize {
  debug_assert_eq!(
    w & TERM_TAG_MASK,
    PrimaryTag::HEADER.get(),
    "Boxed header with arity must have HEADER tag"
  );
  w >> TERM_TAG_BITS
}
