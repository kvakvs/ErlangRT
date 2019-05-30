//! Copying Garbage Collector
//! A simple implementation of half-heap copying (evicting) garbage collector
//! somewhat similar to what is used in Erlang/OTP.
use crate::{
  emulator::heap::{gc_trait::TGc, heap_trait::THeap, *},
  fail::RtResult,
};

pub struct CopyingGc {}

impl TGc for CopyingGc {
  fn new() -> Self {
    Self {}
  }

  fn garbage_collect(_heap: &THeap, mut roots: Box<TRootIterator>) -> RtResult<()> {
    println!("Copying GC");
    roots.roots_begin();
    loop {
      let r = roots.roots_next();
      if r.is_null() {
        break;
      }
      println!("root: {:?}", unsafe { *r });
    }
    unimplemented!("Copying GC")
  }
}
