use crate::{
  defs::sizes::WordSize,
  emulator::heap::Heap,
  fail::RtResult,
  term::{boxed, lterm::*},
};

/// Helper which allows building lists forward or backwards.
///
/// 1. Create ListBuilder with the heap where you want to build.
/// 2. Prepend or append elements, both operations are efficient because
///   the tail pointer is stored.
/// 3. Finalize by requesting the term value of the newly created list.
pub struct ListBuilder {
  // first cell where the building started (used to make the list term, also
  // used to prepend to list)
  pub head_p: *mut boxed::Cons,
  // last cell (used to append to list)
  pub tail_p: *mut boxed::Cons,
  // because i can't into lifetimes :( but it lives short anyway
  heap: *mut Heap,
}

impl ListBuilder {
  pub unsafe fn new(heap: *mut Heap) -> RtResult<ListBuilder> {
    Ok(ListBuilder {
      head_p: core::ptr::null_mut(),
      tail_p: core::ptr::null_mut(),
      heap,
    })
  }

  /// Creates a new cons cell to grow the list either back or forward
  #[inline]
  unsafe fn make_cell(&self) -> RtResult<*mut boxed::Cons> {
    (*self.heap).alloc::<boxed::Cons>(WordSize::new(2), true)
  }

  /// Build list forward: Set current tail to a newly allocated cons (next cell).
  /// New cell becomes the current.
  /// Remember to terminate with NIL.
  pub unsafe fn append(&mut self, val: LTerm) -> RtResult<()> {
    if self.head_p.is_null() {
      // First cell in the list, make it the only cell in list
      self.tail_p = self.make_cell()?;
      self.head_p = self.tail_p;
    } else {
      // Link old tail to new cell
      let new_cell = self.make_cell()?;
      (*self.tail_p).set_tl(LTerm::make_cons(new_cell));
      self.tail_p = new_cell;
    }
    (*self.tail_p).set_hd(val);
    Ok(())
  }

  /// Build list back: Create a new cons, where tail points to current.
  /// New previous cell becomes the current.
  /// Remember to terminate the first cell of the list with NIL.
  pub unsafe fn prepend(&mut self, val: LTerm) -> RtResult<()> {
    if self.head_p.is_null() {
      self.head_p = self.make_cell()?;
      self.tail_p = self.head_p;
    } else {
      let new_cell = self.make_cell()?;
      (*new_cell).set_tl(LTerm::make_cons(self.head_p));
      self.head_p = new_cell;
    }
    (*self.head_p).set_hd(val);
    Ok(())
  }

  pub unsafe fn set_tail(&mut self, tl: LTerm) {
    (*self.tail_p).set_tl(tl)
  }

  pub fn make_term(&self) -> LTerm {
    LTerm::make_cons(self.head_p)
  }

  pub unsafe fn make_term_with_tail(&mut self, tail: LTerm) -> LTerm {
    // Cannot set tail if no cells were allocated
    assert!(!self.head_p.is_null());
    self.set_tail(tail);
    LTerm::make_cons(self.head_p)
  }
}

/// A helper which takes a heap and a UTF-8 string, and creates Erlang string
/// of integer unicode codepoints, one per character.
pub unsafe fn build_erlstr_from_utf8(s: &str, hp: &mut Heap) -> RtResult<LTerm> {
  let mut lb = ListBuilder::new(hp)?;
  for (_pos, ch) in s.char_indices() {
    lb.append(LTerm::make_small_unsigned(ch as usize));
  }
  Ok(lb.make_term())
}