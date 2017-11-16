use num;


/// A generic trait used to hide Raw Tuple interface and decouple tuple parsing
/// from actual tuple implementation.
pub trait ITupleBuilder<TermType: Copy> {
  unsafe fn set_element_base0(&mut self, i: usize, val: TermType);
  fn make_term(&self) -> TermType;
}


/// A forward list builder (works by setting head element in a cons and then
/// allocating another cons for a tail or closing it with a NIL)
pub trait IListBuilder<TermType: Copy> {
  /// Sets head of the current element pointer. Call `next()` to create another
  /// cons cell or call `end()` to close it with a NIL value.
  unsafe fn set(&mut self, val: TermType);

  /// Sets current cell to be a newly allocated cons cell, and sets the tail of
  /// a previous cell to it `[_h | t = new cell]`.
  unsafe fn next(&mut self);

  /// Sets tail of the current cell to possibly nil or another value thus
  /// ending the list.
  unsafe fn end(&mut self, tl: TermType);

  /// Finalize the work and return the boxed pointer to the _first_ element
  /// from where the building started.
  fn make_term(&self) -> TermType;
}


/// A generic trait used to decouple external term format parser from the
/// term implementation in ErlangRT.
pub trait ITermBuilder<TermType: Copy> {
  /// Build a bignum object from a `num::BigInt`.
  fn create_bignum(&self, n: num::BigInt) -> TermType;

  /// Build a binary from bytes.
  fn create_binary(&mut self, b: &[u8]) -> TermType;

  /// Create an atom from a string or return an existing atom.
  fn create_atom_str(&self, a: &str) -> TermType;

  /// Create a NIL \[\] term.
  fn create_nil(&self) -> TermType;

  /// Create a small signed integer (immediate).
  fn create_small_s(&self, n: isize) -> TermType;

  /// Create a {} term.
  fn create_empty_binary(&self) -> TermType;

  /// Get a proxy object for building a tuple.
  fn create_tuple_builder(&mut self, sz: usize) -> Box<ITupleBuilder<TermType>>;

  /// Get a proxy object for building a list (forward builder).
  fn create_list_builder(&mut self) -> Box<IListBuilder<TermType>>;
}
