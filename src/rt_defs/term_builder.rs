use num;


/// A generic trait used to hide Raw Tuple interface and decouple tuple parsing
/// from actual tuple implementation.
pub trait ITupleBuilder<TermT: Copy> {
  unsafe fn set_element_base0(&mut self, i: usize, val: TermT);

  fn make_term(&self) -> TermT;
}


/// A forward list builder (works by setting head element in a cons and then
/// allocating another cons for a tail or closing it with a NIL)
pub trait IListBuilder<TermT: Copy> {
  /// Sets head of the current element pointer. Call `next()` to create another
  /// cons cell or call `end()` to close it with a NIL value.
  unsafe fn set(&mut self, val: TermT);

  /// Sets current cell to be a newly allocated cons cell, and sets the tail of
  /// a previous cell to it `[_h | t = new cell]`.
  unsafe fn next(&mut self);

  /// Sets tail of the current cell to possibly nil or another value thus
  /// ending the list.
  unsafe fn end(&mut self, tl: TermT);

  /// Finalize the work and return the boxed pointer to the _first_ element
  /// from where the building started.
  fn make_term(&self) -> TermT;
}


/// A generic trait used to decouple external term format parser from the
/// term implementation in ErlangRT.
pub trait ITermBuilder {
  type TermT: Copy;
  type TupleBuilderT: ITupleBuilder<Self::TermT>;
  type ListBuilderT: IListBuilder<Self::TermT>;

  /// Build a bignum object from a `num::BigInt`.
  unsafe fn create_bignum(&self, n: num::BigInt) -> Self::TermT;

  /// Build a binary from bytes.
  unsafe fn create_binary(&mut self, b: &[u8]) -> Self::TermT;

  /// Create an atom from a string or return an existing atom.
  fn create_atom_str(&self, a: &str) -> Self::TermT;

  /// Create a NIL \[\] term.
  fn create_nil(&self) -> Self::TermT;

  /// Create a small signed integer (immediate).
  fn create_small_s(&self, n: isize) -> Self::TermT;

  /// Create a {} term.
  fn create_empty_binary(&self) -> Self::TermT;

  /// Get a proxy object for building a tuple.
  fn create_tuple_builder(&mut self, sz: usize) -> Self::TupleBuilderT;

  /// Get a proxy object for building a list (forward builder).
  fn create_list_builder(&mut self) -> Self::ListBuilderT;
}
