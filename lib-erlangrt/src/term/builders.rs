use crate::{
  emulator::{gen_atoms, heap::Heap},
  fail::RtResult,
  term::{lterm::Term, term_builder::TermBuilder},
};
use std::slice;

pub fn make_badfun(arg: Term, hp: &mut Heap) -> RtResult<Term> {
  let slice_of_one = unsafe { slice::from_raw_parts(&arg, 1) };
  make_badfun_n(slice_of_one, hp)
}

/// Create a `{badfun, ...}` tuple where `badfun` is followed by multiple args.
pub fn make_badfun_n(args: &[Term], hp: &mut Heap) -> RtResult<Term> {
  let mut tb = TermBuilder::new(hp);
  let val = tb.create_tuple_builder(1 + args.len())?;
  unsafe {
    val.set_element(0, gen_atoms::BADFUN);
    let mut i = 1usize;
    for arg in args {
      val.set_element(i, *arg);
      i += 1;
    }
  }
  Ok(val.make_term())
}
