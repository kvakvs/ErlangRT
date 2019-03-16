//! The classic BEAM design approach is to copy terms to the new owning heap
//! when an object changes its owner process.
// TODO: Smarter approach with refcounted movable objects or use shared heap or something else
use crate::{
  emulator::heap::Heap,
  fail::RtResult,
  term::{
    boxed,
    term_builder::{ListBuilder, TupleBuilder},
    value::{self, Term},
  },
};

/// Copies term to another heap.
pub fn copy_to(term: Term, hp: &mut Heap) -> RtResult<Term> {
  match term.get_term_tag() {
    value::PrimaryTag::BOX_PTR => unsafe { copy_boxed_to(term, hp) },
    value::PrimaryTag::HEADER => {
      panic!("Attempt to copy header value");
    }
    value::PrimaryTag::CONS_PTR => unsafe { copy_cons_to(term, hp) },
    value::PrimaryTag::SMALL_INT
    | value::PrimaryTag::ATOM
    | value::PrimaryTag::LOCAL_PID
    | value::PrimaryTag::LOCAL_PORT => Ok(term),
    value::PrimaryTag::SPECIAL => match term.get_special_tag() {
      value::SPECIALTAG_CONST => Ok(term),
      _ => panic!("Attempt to copy a special value: {}", term),
    },
    t => panic!("Not sure how to copy term with {:?}", t),
  }
}

/// For each list element copy it to a new element in the destination heap.
/// Also copy the tail element.
/// Returns: `RtResult<copied_term>`
unsafe fn copy_cons_to(lst: Term, hp: &mut Heap) -> RtResult<Term> {
  let mut lb = ListBuilder::new(hp)?;

  let tail_el_result = value::cons::for_each(lst, |el| {
    // Recurse into copy, for each list element
    lb.append(copy_to(el, hp)?)?;
    Ok(())
  })?;

  if let Some(tail_el) = tail_el_result {
    return Ok(lb.make_term_with_tail(tail_el));
  }

  Ok(lb.make_term())
}

unsafe fn copy_boxed_to(term: Term, hp: &mut Heap) -> RtResult<Term> {
  let header_ptr = term.get_box_ptr::<boxed::BoxHeader>();
  let trait_ptr = (*header_ptr).get_trait_ptr();
  let box_type = (*trait_ptr).get_type();

  match box_type {
    boxed::BOXTYPETAG_TUPLE => {
      let tuple_p = header_ptr as *const boxed::Tuple;
      let arity = (*tuple_p).get_arity();
      let tb = TupleBuilder::with_arity(arity, hp)?;
      for i in 0..arity {
        let element = (*tuple_p).get_element(i);
        let copied = copy_to(element, hp)?;
        tb.set_element(i, copied);
      }
      return Ok(tb.make_term());
    }
    boxed::BOXTYPETAG_BIGINTEGER => {}
    boxed::BOXTYPETAG_EXTERNALPID => {}
    boxed::BOXTYPETAG_EXTERNALREF => {}
    boxed::BOXTYPETAG_EXTERNALPORT => {}
    boxed::BOXTYPETAG_CLOSURE => {}
    boxed::BOXTYPETAG_FLOAT => {}
    boxed::BOXTYPETAG_IMPORT => {}
    boxed::BOXTYPETAG_EXPORT => {}
    boxed::BOXTYPETAG_MAP => {}
    boxed::BOXTYPETAG_BINARY => {}
    _other => {}
  }

  panic!("Don't know how to copy {}", term);
}
