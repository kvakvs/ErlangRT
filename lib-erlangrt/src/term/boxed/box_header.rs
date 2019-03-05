use crate::{
  defs::*,
  term::{
    boxed::trait_interface::TBoxed,
    lterm::{TERMTAG_HEADER, TERM_TAG_BITS},
  },
};

/// Term header in memory, followed by corresponding data. The first header word
/// is parsed just like any term, tag bits are set to TERMTAG_HEADER.
pub struct BoxHeader {
  /// Format is <arity> <TAG_HEADER:TERM_TAG_BITS>
  header_word: Word,
  /// Pointer to TBoxed trait vtable depending on the type. Necessary for
  /// reconstructing the fat trait pointer
  trait_vtab: *mut (),
}

impl BoxHeader {
  pub fn new<TraitType>(arity: Word) -> BoxHeader
  where
    TraitType: TBoxed,
  {
    let mut header_word = arity;
    header_word <<= TERM_TAG_BITS;
    header_word |= TERMTAG_HEADER.get();

    // Extract and store vtable pointer from the TBoxed trait object
    let trait_ptr = core::ptr::null_mut::<TraitType>() as *mut TBoxed;
    let trait_obj: core::raw::TraitObject = unsafe { core::mem::transmute(trait_ptr) };
    println!("creating box header, vtab={:p}", trait_obj.vtable);
    BoxHeader {
      header_word,
      trait_vtab: trait_obj.vtable,
    }
  }

  pub const fn storage_size() -> WordSize {
    ByteSize::new(core::mem::size_of::<Self>()).get_words_rounded_up()
  }

  #[inline]
  pub fn get_trait_ptr(&self) -> *const TBoxed {
    let trait_obj = core::raw::TraitObject {
      data: self as *const Self as *mut (),
      vtable: self.trait_vtab,
    };
    unsafe { core::mem::transmute(trait_obj) }
  }

  #[inline]
  pub fn get_trait_ptr_mut(&mut self) -> *mut TBoxed {
    let trait_obj = core::raw::TraitObject {
      data: self as *mut Self as *mut (),
      vtable: self.trait_vtab,
    };
    unsafe { core::mem::transmute(trait_obj) }
  }

  pub fn get_arity(&self) -> usize {
    headerword_to_arity(self.header_word)
  }
}

/// For a header word value, extract bits with arity
/// Format is <arity> <boxtype:BOXTYPE_TAG_BITS> <TAG_HEADER:TERM_TAG_BITS>
pub fn headerword_to_arity(w: Word) -> usize {
  w >> TERM_TAG_BITS
}

// pub const fn headerword_to_boxtype(w: Word) -> BoxType {
//  BoxType((w >> TERM_TAG_BITS) & BOXTYPE_TAG_MASK)
//}
