//! The classic BEAM design approach is to copy terms to the new owning heap
//! when an object changes its owner process.
// TODO: Smarter approach with refcounted movable objects or use shared heap or something else
use crate::{
  emulator::heap::Heap,
  fail::RtResult,
  term::{boxed, lterm::*, term_builder::ListBuilder},
};
use crate::term::term_builder::TupleBuilder;

/// Copies term to another heap.
pub fn copy_to(term: LTerm, hp: &mut Heap) -> RtResult<LTerm> {
  match term.get_term_tag() {
    TERMTAG_BOXED => unsafe {
      copy_boxed_to(term, hp)
    },
    TERMTAG_HEADER => {
      panic!("Attempt to copy header value");
    }
    TERMTAG_CONS => unsafe { copy_cons_to(term, hp) },
    TERMTAG_SMALL | TERMTAG_ATOM | TERMTAG_LOCALPID | TERMTAG_LOCALPORT => Ok(term),
    TERMTAG_SPECIAL => match term.get_special_tag() {
      SPECIALTAG_CONST => Ok(term),
      _ => panic!("Attempt to copy a special value: {}", term),
    },
    t => panic!("Not sure how to copy term with {:?}", t),
  }
}

/// For each list element copy it to a new element in the destination heap.
/// Also copy the tail element.
/// Returns: `RtResult<copied_term>`
unsafe fn copy_cons_to(lst: LTerm, hp: &mut Heap) -> RtResult<LTerm> {
  let mut lb = ListBuilder::new(hp)?;

  let tail_el_result = cons::for_each(lst, |el| {
    // Recurse into copy, for each list element
    lb.set(copy_to(el, hp)?);
    lb.next()?;
    Ok(())
  })?;

  if let Some(tail_el) = tail_el_result {
    lb.end(tail_el);
  } else {
    lb.end(LTerm::nil());
  }

  Ok(lb.make_term())
}

unsafe fn copy_boxed_to(term: LTerm, hp: &mut Heap) -> RtResult<LTerm> {
  let box_p = term.get_box_ptr::<boxed::BoxHeader>();
  match (*box_p).get_tag() {
    boxed::BOXTYPETAG_TUPLE => {
      let tuple_p = box_p as *const boxed::Tuple;
      let arity = (*tuple_p).get_arity();
      let tb = TupleBuilder::with_arity(hp, arity)?;
      for i in 0..arity {
        let element = boxed::Tuple::get_element_base0(tuple_p, i);
        let copied = copy_to(element, hp)?;
        tb.set_element_base0(i, copied);
      }
      return Ok(tb.make_term());
    },
    boxed::BOXTYPETAG_BIGINTEGER => {},
    boxed::BOXTYPETAG_EXTERNALPID => {},
    boxed::BOXTYPETAG_EXTERNALREF => {},
    boxed::BOXTYPETAG_EXTERNALPORT => {},
    boxed::BOXTYPETAG_CLOSURE => {},
    boxed::BOXTYPETAG_FLOAT => {},
    boxed::BOXTYPETAG_IMPORT => {},
    boxed::BOXTYPETAG_EXPORT => {},
    boxed::BOXTYPETAG_MAP => {},
    boxed::BOXTYPETAG_BINARY => {},
    _other => {},
  }

  panic!("Don't know how to copy {}", term);
}
