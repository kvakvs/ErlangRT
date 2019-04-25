use crate::{
  emulator::heap::heap_trait::THeap,
  fail::RtResult,
  term::{boxed, value::Term},
};

/// Map builder allocates necessary space on the given heap and allows
/// adding keys and values as necessary.
///
/// 1. Create MapBuilder with the heap where you want to build.
/// 2. Call `add(key, value)`
/// 3. Finalize by requesting the term value of a newly built map.
pub struct MapBuilder {
  // because i can't into lifetimes :( but it lives short anyway
  // heap: *mut Heap,
  p: *mut boxed::Map,
}

impl MapBuilder {
  pub fn new(heap: &mut THeap, size_hint: usize) -> RtResult<Self> {
    let p = boxed::Map::create_into(heap, size_hint)?;
    Ok(Self { p })
  }

  pub unsafe fn add(&mut self, key: Term, value: Term) -> RtResult<()> {
    boxed::Map::add(self.p, key, value)?;
    Ok(())
  }

  pub fn make_term(&mut self) -> Term {
    Term::make_boxed(self.p)
  }
}
