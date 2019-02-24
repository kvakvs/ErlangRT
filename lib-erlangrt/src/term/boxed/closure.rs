use crate::{
  defs::{Arity, ByteSize, Word, WordSize},
  emulator::{
    code::pointer::{CodePtr, VersionedCodePtr},
    code_srv::CodeServer,
    function::FunEntry,
    heap::Heap,
    mfa::MFArity,
  },
  fail::{RtErr, RtResult},
  term::{
    boxed::{BoxHeader, BOXTYPETAG_CLOSURE},
    lterm::*,
  },
};
use core::mem::size_of;

const fn module() -> &'static str {
  "closure: "
}

/// Boxed `Closure` is placed on heap and referred via LTerm::p
#[allow(dead_code)]
pub struct Closure {
  pub header: BoxHeader,

  pub mfa: MFArity,
  pub dst: Option<VersionedCodePtr>,
  // Frozen value count, values follow in memory after the Closure struct
  // must be word size to avoid alignment of the following data
  pub nfrozen: usize,
}

impl Closure {
  #[inline]
  const fn storage_size(nfrozen: Word) -> WordSize {
    ByteSize::new(size_of::<Closure>())
      .words_rounded_up()
      .add(nfrozen)
  }

  fn new(mfa: MFArity, nfrozen: usize) -> Closure {
    let arity = Closure::storage_size(nfrozen).words() - 1;
    Closure {
      header: BoxHeader::new(BOXTYPETAG_CLOSURE, arity),
      mfa,
      dst: None,
      nfrozen: nfrozen as Arity,
    }
  }

  pub unsafe fn create_into(
    hp: &mut Heap,
    fe: &FunEntry,
    frozen: &[LTerm],
  ) -> RtResult<LTerm> {
    let n_words = Closure::storage_size(fe.nfrozen);
    let this = hp.alloc::<Closure>(n_words, false)?;

    assert_eq!(frozen.len(), fe.nfrozen as usize);
    println!(
      "{}new closure: {} frozen={} nfrozen={}",
      module(),
      fe.mfa,
      frozen.len(),
      fe.nfrozen
    );

    core::ptr::write(this, Closure::new(fe.mfa, fe.nfrozen));

    assert_eq!(frozen.len(), fe.nfrozen as usize);
    // step 1 closure forward, which will point exactly at the frozen location
    let dst = Self::get_frozen_mut(this);
    dst.copy_from_slice(frozen);

    Ok(LTerm::make_boxed(this))
  }

  #[allow(dead_code)]
  pub unsafe fn const_from_term(t: LTerm) -> RtResult<*const Self> {
    helper_get_const_from_boxed_term::<Self>(
      t,
      BOXTYPETAG_CLOSURE,
      RtErr::BoxedIsNotAClosure,
    )
  }

  #[allow(dead_code)]
  pub unsafe fn mut_from_term(t: LTerm) -> RtResult<*mut Self> {
    helper_get_mut_from_boxed_term::<Self>(
      t,
      BOXTYPETAG_CLOSURE,
      RtErr::BoxedIsNotAClosure,
    )
  }

  /// Given a closure, find new value for the code pointer and update the
  /// closure. Return: the pointer.
  pub unsafe fn update_location(&mut self, c_srv: &mut CodeServer) -> RtResult<CodePtr> {
    let new_dst = c_srv.lookup_beam_code_versioned(&self.mfa)?;
    let ptr = new_dst.ptr;
    self.dst = Some(new_dst);
    Ok(ptr)
  }

  /// Return a const pointer to the memory word after the closure, where you
  /// can access frozen values (read only).
  /// It is responsibility of the caller to forget the slice as soon as possible.
  #[inline]
  pub unsafe fn get_frozen(this: *const Closure) -> &'static [LTerm] {
    let nfrozen = (*this).nfrozen;
    let frozen_ptr = this.add(1) as *const LTerm;
    core::slice::from_raw_parts(frozen_ptr, nfrozen)
  }

  /// Return a mutable pointer to the memory word after the closure, where you
  /// can access frozen values (read/write).
  /// It is responsibility of the caller to forget the slice as soon as possible.
  #[inline]
  pub unsafe fn get_frozen_mut(this: *mut Closure) -> &'static mut [LTerm] {
    let nfrozen = (*this).nfrozen;
    let frozen_ptr = this.add(1) as *mut LTerm;
    core::slice::from_raw_parts_mut(frozen_ptr, nfrozen)
  }
}
