//use rt_defs::heap::IHeap;

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
  //// Using mutability build list forward creating many cells and linking them
  //let cell0 = cell.clone();
  //let n_elem_minus_one = n_elem - 1;
  //
  //for i in 0..n_elem {
  //let elem = decode_naked(r, hp)?;
  //unsafe { cell.set_hd(elem) }
  //
  //// Keep building forward
  //if i < n_elem_minus_one {
  //let new_cell = hp.allocate_cons().unwrap();
  //unsafe { cell.set_tl(new_cell.make_cons()) }
  //cell = new_cell;
  //}
  //}
  //
  //let tl = decode_naked(r, hp)?;
  //unsafe { cell.set_tl(tl) }
  //
  //Ok(cell0.make_cons())

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
  //let rbig = unsafe { HOBignum::place_into(heap, big)? };
  //Ok(HOBignum::make_term(rbig))
  fn create_bignum(&self, n: num::BigInt) -> TermType;

  //unsafe {
  //let rbin = HOBinary::place_into(hp, n_bytes as Word)?;
  //HOBinary::store(rbin, &data);
  //return Ok(HOBinary::make_term(rbin))
  //}
  fn create_binary(&mut self, b: &[u8]) -> TermType;

  //atom::from_str(&val)
  fn create_atom_str(&self, a: &str) -> TermType;

  fn create_nil(&self) -> TermType;

  fn create_small_s(&self, n: isize) -> TermType;

  fn create_empty_binary(&self) -> TermType;

  //hp.allocate_tuple(size)?
  fn create_tuple_builder(&mut self, sz: usize) -> Box<ITupleBuilder<TermType>>;

  // hp.allocate_cons().unwrap();
  fn create_list_builder(&mut self) -> Box<IListBuilder<TermType>>;
}
