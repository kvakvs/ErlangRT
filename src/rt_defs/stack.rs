use super::Word;
use emulator::heap::HeapError;


/// Defines common interface for stack operations. Implemented by Heaps.
pub trait IStack<TermType: Copy> {
  fn stack_have(&self, need: Word) -> bool;

  //  pub fn stack_alloc(&mut self, need: Word) -> Hopefully<()> {

  /// Allocate stack cells without checking. Call `stack_have(n)` beforehand.
  fn stack_alloc_unchecked(&mut self, need: Word);

  // TODO: Add unsafe push without range checks (batch check+multiple push)
  //  pub fn stack_push(&mut self, val: Word) -> Hopefully<()> {

  fn stack_info(&self);

  /// Push a value to stack without checking. Call `stack_have(1)` beforehand.
  fn stack_push_unchecked(&mut self, val: Word);

  /// Check whether `y+1`-th element can be found in stack
  fn stack_have_y(&self, y: Word) -> bool;

  fn stack_set_y(&mut self, index: Word, val: TermType) -> Result<(), HeapError>;

  fn stack_get_y(&self, index: Word) -> Result<TermType, HeapError>;

  fn stack_depth(&self) -> Word;

  /// Take `cp` from stack top and deallocate `n+1` words of stack.
  fn stack_deallocate(&mut self, n: Word) -> TermType;
}
