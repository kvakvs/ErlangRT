use crate::{
  emulator::{gen_atoms, heap::Heap},
  fail::RtResult,
  term::{lterm::LTerm, term_builder::TermBuilder},
};
use std::slice;

pub fn make_badfun(arg: LTerm, hp: &mut Heap) -> RtResult<LTerm> {
  let slice_of_one = unsafe { slice::from_raw_parts(&arg, 1) };
  make_badfun_n(slice_of_one, hp)
}

/// Create a `{badfun, ...}` tuple where `badfun` is followed by multiple args.
pub fn make_badfun_n(args: &[LTerm], hp: &mut Heap) -> RtResult<LTerm> {
  let mut tb = TermBuilder::new(hp);
  let val = tb.create_tuple_builder(1 + args.len())?;
  unsafe {
    val.set_element_base0(0, gen_atoms::BADFUN);
    let mut i = 1usize;
    for arg in args {
      val.set_element_base0(i, *arg);
      i += 1;
    }
  }
  Ok(val.make_term())
}
