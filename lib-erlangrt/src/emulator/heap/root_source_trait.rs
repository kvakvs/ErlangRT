use crate::term::Term;

/// Mutable root is a mutable slice which will be read, collected and then
/// updated with new locations of the data.
/// Used in garbage collect call.
pub type MutableRoot = *mut Term;

pub trait TRootIterator {
  fn roots_begin(&mut self);
  fn roots_next(&mut self) -> MutableRoot;
}

/// Roots source is an object which is able to provide an array of mutable pointers
/// to term values, so called ROOTS for the garbage collector. The root set
/// serves as a starting point for tracing the heap value liveness during GC.
pub trait TRootSource {
  fn roots_get_iterator(&mut self) -> Box<TRootIterator>;
}

/// Root source which takes root from a contiguous memory array with start and end.
pub struct ArrayRootIterator {
  start: *mut Term,
  stop: *mut Term,
  position: *mut Term,
}

impl ArrayRootIterator {
  pub fn new(src: &mut [Term]) -> Self {
    let start = src.as_mut_ptr();
    unsafe {
      Self {
        start,
        stop: start.add(src.len()),
        position: start,
      }
    }
  }
}

impl TRootIterator for ArrayRootIterator {
  #[inline]
  fn roots_begin(&mut self) {
    self.position = self.start;
  }

  /// Step forward, return null if reached the end.
  #[inline]
  fn roots_next(&mut self) -> *mut Term {
    if self.position >= self.stop {
      return core::ptr::null_mut();
    }
    unsafe {
      self.position = self.position.add(1);
      self.position
    }
  }
}
