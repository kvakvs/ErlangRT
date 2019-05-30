use crate::{
  fail::RtResult,
  term::{boxed, Term},
};

// Return byte size of a binary, rounded up.
define_nativefun!(_vm, _proc, args,
  name: "erlang:byte_size/1", struct_name: NfErlangByteSize1, arity: 1,
  invoke: { byte_size_1(t) },
  args: binary(t),
);

#[inline]
fn byte_size_1(t: Term) -> RtResult<Term> {
  if t == Term::empty_binary() {
    return Ok(Term::make_small_unsigned(0));
  }

  let bin_ptr = unsafe { boxed::Binary::get_trait_from_term(t) };
  let bin_size = unsafe { (*bin_ptr).get_byte_size() };
  Ok(Term::make_small_unsigned(bin_size.bytes()))
}

// Return bit size of a binary.
define_nativefun!(_vm, _proc, args,
  name: "erlang:bit_size/1", struct_name: NfErlangBitSize1, arity: 1,
  invoke: { bit_size_1(t) },
  args: binary(t),
);

#[inline]
fn bit_size_1(t: Term) -> RtResult<Term> {
  if t == Term::empty_binary() {
    return Ok(Term::make_small_unsigned(0));
  }

  let bin_ptr = unsafe { boxed::Binary::get_trait_from_term(t) };
  let bin_size = unsafe { (*bin_ptr).get_bit_size() };
  Ok(Term::make_small_unsigned(bin_size.bits))
}
