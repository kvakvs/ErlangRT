//! Term ordering and classification.

use defs::Word;
use term::immediate;
use term::lterm::*;
use term::primary;
use term::raw::heapobj::HeapObjClass;


fn module() -> &'static str { "classify: " }

/// Enum defines term classification for order comparisons. Use `cmp` on this
/// enum to get relative order for two terms. Enum values are listed in
/// comparison order according to
/// [Term Comparisons](http://erlang.org/doc/reference_manual/expressions.html)
///
/// The order from the documentation is:
///
/// * `number` is less than
/// * `atom` is less than
/// * `reference` is less than
/// * `fun` is less than
/// * `port` is less than
/// * `pid` is less than
/// * `tuple` is less than
/// * `map` is less than
/// * `nil` is less than
/// * `list` is less than
/// * `bit string` (`binary`).
///
#[derive(Ord, PartialOrd, Eq, PartialEq, Copy, Clone)]
#[allow(dead_code)]
pub enum TermClass {
  Number,
  Atom,
  Ref,
  Fun,
  Port,
  Pid,
  Tuple,
  Map,
  Nil,
  Cons,
  Binary,
  // Means the value should not be used in comparisons but here it is anyway
  Special_,
}


pub fn classify_term(t: LTerm) -> TermClass {
  let v = t.raw();
  match primary::get_tag(v) {
    primary::TAG_BOX => unsafe { classify_box(t) },
    primary::TAG_HEADER => TermClass::Special_, // won't look into the header
    primary::TAG_IMMED => classify_immed(v, t),
    primary::TAG_CONS => TermClass::Cons,
    _ => panic!("{}Invalid primary tag", module())
  }
}


/// Given term's raw value `v` and the term itself, try and figure out the
/// classification value for this term.
#[inline]
unsafe fn classify_box(t: LTerm) -> TermClass {
  if t.is_cp() {
    //panic!("Can't classify a CP value")
    return TermClass::Special_
  }
  let p = t.box_ptr();
  classify_header(*p, p)
}


#[inline]
unsafe fn classify_header(v: Word, p: *const Word) -> TermClass {
  let h_tag = primary::header::get_tag(v);
  match h_tag {
    primary::header::TAG_HEADER_TUPLE => TermClass::Tuple,
    primary::header::TAG_HEADER_REF => TermClass::Ref,
    primary::header::TAG_HEADER_FUN => TermClass::Fun,
    primary::header::TAG_HEADER_FLOAT => TermClass::Number,
    primary::header::TAG_HEADER_HEAPOBJ => {
      let ho_ptr = p.offset(1);
      let hoclass = *(ho_ptr) as *const HeapObjClass;
      (*hoclass).term_class
    },
    primary::header::TAG_HEADER_EXTPID => TermClass::Pid,
    primary::header::TAG_HEADER_EXTPORT => TermClass::Port,
    primary::header::TAG_HEADER_EXTREF => TermClass::Ref,
    _ => panic!("classify: Unexpected header tag {} value {}",
                h_tag, primary::get_value(v))
  }
}


/// Given term's raw value `v` and the term itself, parse its immediate subtags
/// and figure out its classification value.
#[inline]
fn classify_immed(v: Word, t: LTerm) -> TermClass {
  match immediate::get_imm1_tag(v) {
    immediate::TAG_IMM1_SMALL => TermClass::Number,
    immediate::TAG_IMM1_PID => TermClass::Pid,
    immediate::TAG_IMM1_PORT => TermClass::Port,
    immediate::TAG_IMM1_IMM2 => {
      match immediate::get_imm2_tag(v) {
        immediate::TAG_IMM2_CATCH => TermClass::Special_,
        immediate::TAG_IMM2_SPECIAL => {
          if t.is_nil() {
            TermClass::Nil
          } else if t.is_empty_tuple() {
            TermClass::Tuple
          } else if t.is_empty_binary() {
            TermClass::Binary
          } else {
            TermClass::Special_
          }
        },
        immediate::TAG_IMM2_ATOM => TermClass::Atom,
        immediate::TAG_IMM2_IMM3 => TermClass::Special_,
        _ => panic!("{}Invalid primary tag", module())
      } // end match imm2
    },
    _ => panic!("{}Invalid primary tag", module())
  } // end match imm1
}
