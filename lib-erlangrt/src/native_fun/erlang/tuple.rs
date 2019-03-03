use crate::{
  fail::{self, RtResult},
  term::{boxed, lterm::LTerm},
};

/// Return size of a tuple or a binary object.
define_nativefun!(_vm, _proc, args,
  name: "erlang:size/1", struct_name: NfErlangSize1, arity: 1,
  invoke: { size_1(t) },
  args: term(t),
);

#[inline]
fn size_1(t: LTerm) -> RtResult<LTerm> {
  if t.is_tuple() {
    let t_ptr = t.get_tuple_ptr();
    let arity = unsafe { (*t_ptr).get_arity() };
    return Ok(LTerm::make_small_unsigned(arity));
  } else if t.is_binary() {
    let bin_ptr = unsafe { boxed::Binary::get_trait_from_term(t) };
    let bin_size = unsafe { (*bin_ptr).get_byte_size() };
    return Ok(LTerm::make_small_unsigned(bin_size.bytes()));
  } else {
    return fail::create::badarg();
  }
}
