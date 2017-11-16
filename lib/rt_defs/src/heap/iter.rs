//use super::ptr::DataPtr;

/// A heap iterator. Not very `std::iter::Iterator` compatible but simple.
pub trait IHeapIterator<PtrType> {
  fn next(&mut self) -> Option<PtrType>;
}
