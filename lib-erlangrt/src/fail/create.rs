use crate::{
  defs::exc_type::ExceptionType,
  emulator::{gen_atoms, heap::Heap},
  fail::{Error, RtResult},
  term::{
    builders::make_badfun,
    lterm::{tuple, LTerm},
  },
};

pub fn badmatch_val<T>(val: LTerm, hp: &mut Heap) -> RtResult<T> {
  let badmatch_tuple = tuple::make_tuple2(gen_atoms::BADMATCH, val, hp)?;
  Err(Error::Exception(ExceptionType::Error, badmatch_tuple))
}

pub fn badarity<T>() -> RtResult<T> {
  Err(Error::Exception(ExceptionType::Error, gen_atoms::BADARITY))
}

pub fn badarg<T>() -> RtResult<T> {
  Err(Error::Exception(ExceptionType::Error, gen_atoms::BADARG))
}

pub fn badarg_val<T>(val: LTerm, hp: &mut Heap) -> RtResult<T> {
  let badarg_tuple = tuple::make_tuple2(gen_atoms::BADARG, val, hp)?;
  Err(Error::Exception(ExceptionType::Error, badarg_tuple))
}

pub fn undef<T>() -> RtResult<T> {
  Err(Error::Exception(ExceptionType::Error, gen_atoms::UNDEF))
}

pub fn badfun<T>() -> RtResult<T> {
  Err(Error::Exception(ExceptionType::Error, gen_atoms::BADFUN))
}

pub fn badfun_val<T>(val: LTerm, hp: &mut Heap) -> RtResult<T> {
  let badfun_tuple = make_badfun(val, hp)?;
  Err(Error::Exception(ExceptionType::Error, badfun_tuple))
}
