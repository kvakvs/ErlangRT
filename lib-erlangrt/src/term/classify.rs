//! Term ordering and classification.

use crate::term::{boxed::BoxHeader, lterm::*};

fn module() -> &'static str {
  "classify: "
}

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
#[derive(Ord, PartialOrd, Eq, PartialEq, Copy, Clone)]
#[allow(dead_code)]
pub struct TermClass(usize);

pub const CLASS_NUMBER: TermClass = TermClass(10);
pub const CLASS_ATOM: TermClass = TermClass(20);
#[allow(dead_code)]
pub const CLASS_REF: TermClass = TermClass(30);
pub const CLASS_FUN: TermClass = TermClass(40);
pub const CLASS_PORT: TermClass = TermClass(50);
pub const CLASS_PID: TermClass = TermClass(60);
pub const CLASS_TUPLE: TermClass = TermClass(70);
pub const CLASS_MAP: TermClass = TermClass(80);
pub const CLASS_LIST: TermClass = TermClass(90);
pub const CLASS_BINARY: TermClass = TermClass(100);
/// The SPECIAL class should not be used in comparisons but here it is anyway
pub const CLASS_SPECIAL: TermClass = TermClass(500);


pub fn classify_term(t: Term) -> TermClass {
  // let _v = t.raw();
  match t.get_term_tag() {
    TERMTAG_BOXED => {
      if t.is_cp() {
        CLASS_SPECIAL
      } else {
        unsafe { classify_boxed(t) }
      }
    }
    TERMTAG_CONS => CLASS_LIST,
    TERMTAG_SMALL => CLASS_NUMBER,
    TERMTAG_HEADER => CLASS_SPECIAL, // won't look into the header
    TERMTAG_ATOM => CLASS_ATOM,
    TERMTAG_LOCALPID => CLASS_PID,
    TERMTAG_LOCALPORT => CLASS_PORT,
    TERMTAG_SPECIAL => classify_special(t),
    _ => panic!("{}Invalid primary tag {:?}", module(), t.get_term_tag()),
  }
}

fn classify_special(val: Term) -> TermClass {
  match val.get_special_tag() {
    SPECIALTAG_CONST => {
      if val == Term::nil() {
        CLASS_LIST
      } else if val == Term::empty_binary() {
        CLASS_BINARY
      } else if val == Term::empty_tuple() {
        CLASS_TUPLE
      } else {
        CLASS_SPECIAL
      }
    }
    SPECIALTAG_REGX | SPECIALTAG_REGY | SPECIALTAG_REGFP => CLASS_SPECIAL,
    SpecialTag(unk) => panic!("classify_special: failed for specialtag {}", unk),
  }
}

/// Given term's raw value `v` and the term itself, try and figure out the
/// classification value for this term.
unsafe fn classify_boxed(val: Term) -> TermClass {
  let val_box_ptr = val.get_box_ptr_mut::<BoxHeader>();
  let trait_ptr = (*val_box_ptr).get_trait_ptr();
  (*trait_ptr).get_class()
  //  let box_tag = (*val_box_ptr).get_tag();
  //  match box_tag {
  //    boxed::BOXTYPETAG_TUPLE => CLASS_TUPLE,
  //    boxed::BOXTYPETAG_BINARY => CLASS_BINARY,
  //    boxed::BOXTYPETAG_EXTERNALPID => CLASS_PID,
  //    boxed::BOXTYPETAG_EXTERNALREF => CLASS_REF,
  //    boxed::BOXTYPETAG_CLOSURE => CLASS_FUN,
  //    boxed::BOXTYPETAG_FLOAT => CLASS_NUMBER,
  //    _ => unimplemented!(
  //      "classify: Unexpected boxed_tag={:?} raw={}",
  //      box_tag,
  //      val.raw()
  //    ),
  //  }
}
