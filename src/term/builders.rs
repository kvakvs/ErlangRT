use emulator::gen_atoms;
use emulator::heap::Heap;
use rt_defs::term_builder::*;
use std::slice;
use term::lterm::LTerm;
use term::term_builder::TermBuilder;


pub fn make_badfun(arg: LTerm, hp: &mut Heap) -> LTerm {
  let slice_of_one = unsafe { slice::from_raw_parts(&arg, 1) };
  make_badfun_n(slice_of_one, hp)
}


/// Create a `{badfun, ...}` tuple where `badfun` is followed by multiple args.
pub fn make_badfun_n(args: &[LTerm], hp: &mut Heap) -> LTerm {
  let mut tb = TermBuilder::new(hp);
  let mut val = tb.create_tuple_builder(1 + args.len());
  unsafe {
    val.set_element_base0(0, gen_atoms::BADFUN);
    let mut i = 1usize;
    for arg in args {
      val.set_element_base0(i, *arg);
      i += 1;
    }
  }
  val.make_term()
}


/// Create a `{badmatch, Arg}`.
pub fn make_badmatch(arg: LTerm, hp: &mut Heap) -> LTerm {
  let mut tb = TermBuilder::new(hp);
  let mut val = tb.create_tuple_builder(2);
  unsafe {
    val.set_element_base0(0, gen_atoms::BADMATCH);
    val.set_element_base0(1, arg);
  }
  val.make_term()
}
