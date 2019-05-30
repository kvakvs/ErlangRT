use crate::{
  defs::{Arity, ByteSize, Word, WordSize},
  emulator::{
    code::pointer::{CodePtr, VersionedCodePtr},
    code_srv::CodeServer,
    function::FunEntry,
    heap::{AllocInit, THeap},
    mfa::ModFunArity,
  },
  fail::{RtErr, RtResult},
  term::{
    boxed::{
      boxtype::{self, BoxType},
      trait_interface::TBoxed,
      BoxHeader, BOXTYPETAG_CLOSURE,
    },
    classify,
    value::*,
  },
};
use core::mem::size_of;

const fn module() -> &'static str {
  "closure: "
}

/// Boxed `Closure` is placed on heap and referred via Term::p
#[allow(dead_code)]
pub struct Closure {
  pub header: BoxHeader,

  pub mfa: ModFunArity,
  pub dst: Option<VersionedCodePtr>,
  // Frozen value count, values follow in memory after the Closure struct
  // must be word size to avoid alignment of the following data
  pub nfrozen: usize,
}

impl TBoxed for Closure {
  fn get_class(&self) -> classify::TermClass {
    classify::CLASS_FUN
  }

  fn get_type(&self) -> BoxType {
    boxtype::BOXTYPETAG_CLOSURE
  }

  //  fn inplace_map(&mut self, mapfn: &InplaceMapFn) {
  //    let this_p = self as *mut Closure;
  //    let frozen = unsafe { (*this_p).get_frozen_mut() };
  //    for i in 0..frozen.len() {
  //      frozen[i] = mapfn(this_p as *mut BoxHeader, frozen[i]);
  //    }
  //  }
}

impl Closure {
  #[inline]
  const fn storage_size(nfrozen: Word) -> WordSize {
    ByteSize::new(size_of::<Self>())
      .get_words_rounded_up()
      .add(nfrozen)
  }

  fn new(mfa: ModFunArity, nfrozen: usize) -> Self {
    let storage_size = Self::storage_size(nfrozen) - WordSize::one();
    Self {
      header: BoxHeader::new::<Self>(storage_size),
      mfa,
      dst: None,
      nfrozen: nfrozen as Arity,
    }
  }

  pub unsafe fn create_into(
    hp: &mut THeap,
    fe: &FunEntry,
    frozen: &[Term],
  ) -> RtResult<Term> {
    let n_words = Self::storage_size(fe.nfrozen);
    let this = hp.alloc(n_words, AllocInit::Uninitialized)? as *mut Self;

    assert_eq!(frozen.len(), fe.nfrozen as usize);
    println!(
      "{}new closure: {} frozen={} nfrozen={}",
      module(),
      fe.mfa,
      frozen.len(),
      fe.nfrozen
    );

    this.write(Self::new(fe.mfa, fe.nfrozen));

    assert_eq!(frozen.len(), fe.nfrozen as usize);
    // step 1 closure forward, which will point exactly at the frozen location
    let dst = (*this).get_frozen_mut();
    dst.copy_from_slice(frozen);

    Ok(Term::make_boxed(this))
  }

  #[allow(dead_code)]
  pub unsafe fn const_from_term(t: Term) -> RtResult<*const Self> {
    helper_get_const_from_boxed_term::<Self>(
      t,
      BOXTYPETAG_CLOSURE,
      RtErr::BoxedIsNotAClosure,
    )
  }

  #[allow(dead_code)]
  pub unsafe fn mut_from_term(t: Term) -> RtResult<*mut Self> {
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
  pub unsafe fn get_frozen(&self) -> &'static [Term] {
    let nfrozen = self.nfrozen;
    let frozen_ptr = (self as *const Self).add(1) as *const Term;
    core::slice::from_raw_parts(frozen_ptr, nfrozen)
  }

  /// Return a mutable pointer to the memory word after the closure, where you
  /// can access frozen values (read/write).
  /// It is responsibility of the caller to forget the slice as soon as possible.
  #[inline]
  pub unsafe fn get_frozen_mut(&mut self) -> &'static mut [Term] {
    let nfrozen = self.nfrozen;
    let frozen_ptr = (self as *mut Self).add(1) as *mut Term;
    core::slice::from_raw_parts_mut(frozen_ptr, nfrozen)
  }
}
