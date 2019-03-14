use crate::{
  defs::exc_type::ExceptionType,
  emulator::{gen_atoms, heap::Heap},
  fail::{RtErr, RtResult},
  term::{builders::make_badfun, value::Term},
};

#[inline]
fn generic_fail<T>(err_atom: Term) -> RtResult<T> {
  Err(RtErr::Exception(ExceptionType::Error, err_atom))
}

#[inline]
pub fn generic_tuple2_fail<T>(err_atom: Term, val: Term, hp: &mut Heap) -> RtResult<T> {
  Err(RtErr::Exception(ExceptionType::Error, hp.tuple2(err_atom, val)?))
}

pub fn badmatch_val<T>(val: Term, hp: &mut Heap) -> RtResult<T> {
  generic_tuple2_fail(gen_atoms::BADMATCH, val, hp)
}

pub fn badarity<T>() -> RtResult<T> {
  generic_fail(gen_atoms::BADARITY)
}

pub fn badarg<T>() -> RtResult<T> {
  generic_fail(gen_atoms::BADARG)
}

pub fn badarg_val<T>(val: Term, hp: &mut Heap) -> RtResult<T> {
  generic_tuple2_fail(gen_atoms::BADARG, val, hp)
}

pub fn undef<T>() -> RtResult<T> {
  generic_fail(gen_atoms::UNDEF)
}

pub fn badfun<T>() -> RtResult<T> {
  generic_fail(gen_atoms::BADFUN)
}

pub fn badfun_val<T>(val: Term, hp: &mut Heap) -> RtResult<T> {
  let badfun_tuple = make_badfun(val, hp)?;
  Err(RtErr::Exception(ExceptionType::Error, badfun_tuple))
}

pub fn system_limit<T>() -> RtResult<T> {
  generic_fail(gen_atoms::SYSTEM_LIMIT)
}
