//! Implement format trait (Display) for Term
// Printing low_level Terms as "{}"
use crate::{
  defs::Word,
  emulator::atom,
  term::{
    boxed::{self, box_header::BoxHeader, boxtype},
    value::{cons, *},
  },
};
use core::fmt;

impl fmt::Display for Term {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    if self.is_non_value() {
      return write!(f, "NON_VALUE");
    }
    match self.get_term_tag() {
      TERMTAG_BOXED => unsafe {
        if self.is_cp() {
          write!(f, "CP({:p})", self.get_cp_ptr::<Word>())
        } else {
          let box_ptr = self.get_box_ptr::<boxed::BoxHeader>();
          // `p` can't be null because non_value=0 is checked above
          format_box_contents(box_ptr, f)
        }
      },

      TERMTAG_CONS => unsafe {
        if self.cons_is_ascii_string() {
          format_cons_ascii(*self, f)
        } else {
          format_cons(*self, f)
        }
      },

      TERMTAG_SMALL => write!(f, "{}", self.get_small_signed()),

      TERMTAG_SPECIAL => format_special(*self, f),

      TERMTAG_LOCALPID => write!(f, "Pid<{}>", self.get_term_val_without_tag()),

      TERMTAG_LOCALPORT => write!(f, "Port<{}>", self.get_term_val_without_tag()),

      TERMTAG_ATOM => match atom::to_str(*self) {
        Ok(s) => {
          if atom::is_printable_atom(&s) {
            write!(f, "{}", s)
          } else {
            write!(f, "'{}'", s)
          }
        }
        Err(e) => write!(f, "Atom<printing failed {:?}>", e),
      },

      TERMTAG_HEADER => {
        return write!(f, "Header({})", boxed::headerword_to_arity(self.raw()));
        // format_box_contents(*self, ptr::null(), f)?;
        // write!(f, ")")
      }

      _ => panic!("Primary tag {:?} not recognized", self.get_term_tag()),
    }
  }
} // trait Display

/// Attempt to display contents of a tagged header word and the words which
/// follow it. Arg `p` if not null is used to fetch the following memory words
/// and display more detail.
unsafe fn format_box_contents(
  box_ptr: *const BoxHeader,
  f: &mut fmt::Formatter,
) -> fmt::Result {
  let trait_ptr = (*box_ptr).get_trait_ptr();
  let box_type = (*trait_ptr).get_type();

  match box_type {
    boxtype::BOXTYPETAG_BINARY => {
      let trait_ptr = boxed::Binary::get_trait(trait_ptr as *const boxed::Binary);
      if cfg!(debug_assertions) {
        write!(
          f,
          "{:?}({};{})",
          (*trait_ptr).get_type(),
          (*trait_ptr).get_byte_size(),
          (*trait_ptr).get_bit_size()
        )?;
      }
      boxed::Binary::format(trait_ptr, f)
    }
    boxtype::BOXTYPETAG_BIGINTEGER => write!(f, "Big<>"),
    boxtype::BOXTYPETAG_TUPLE => {
      let tuple_ptr = trait_ptr as *const boxed::Tuple;
      (*tuple_ptr).format(f)
    }
    boxtype::BOXTYPETAG_CLOSURE => {
      let fun_p = trait_ptr as *const boxed::Closure;
      write!(f, "Fun<{}>", (*fun_p).mfa)
    }
    boxtype::BOXTYPETAG_FLOAT => {
      let fptr = trait_ptr as *const boxed::Float;
      write!(f, "{}", (*fptr).value)
    }
    boxtype::BOXTYPETAG_EXTERNALPID => write!(f, "ExtPid<>"),
    boxtype::BOXTYPETAG_EXTERNALPORT => write!(f, "ExtPort<>"),
    boxtype::BOXTYPETAG_EXTERNALREF => write!(f, "ExtRef<>"),
    boxtype::BOXTYPETAG_IMPORT => {
      let iptr = trait_ptr as *const boxed::Import;
      write!(f, "Import<{}>", (*iptr).mfarity)
    }
    boxtype::BOXTYPETAG_EXPORT => {
      let eptr = trait_ptr as *const boxed::Export;
      write!(f, "Export<{}>", (*eptr).exp.mfa)
    }
    boxtype::BOXTYPETAG_BINARY_MATCH_STATE => write!(f, "BinaryMatchState<>"),

    _ => panic!("Unexpected header tag {:?}", box_type),
  }
}

// Formatting helpers
//

fn format_special(term: Term, f: &mut fmt::Formatter) -> fmt::Result {
  match term.get_special_tag() {
    SPECIALTAG_CONST => {
      if term == Term::nil() {
        return write!(f, "[]");
      } else if term.is_non_value() {
        return write!(f, "NON_VALUE");
      } else if term == Term::empty_binary() {
        return write!(f, "<<>>");
      } else if term == Term::empty_tuple() {
        return write!(f, "{{}}");
      }
    }
    SPECIALTAG_REG => {
      let rtag = term.get_reg_tag();
      if rtag == SPECIALREG_X {
        return write!(f, "X{}", term.get_reg_value());
      } else if rtag == SPECIALREG_Y {
        return write!(f, "Y{}", term.get_reg_value());
      } else if rtag == SPECIALREG_FP {
        return write!(f, "F{}", term.get_reg_value());
      } else {
        panic!("Unknown special reg tag")
      }
    }
    SPECIALTAG_OPCODE => return write!(f, "Opcode({})", term.get_special_value()),
    SPECIALTAG_CATCH => return write!(f, "Catch({:p})", term.get_catch_ptr()),
    _ => {}
  }
  write!(
    f,
    "Special(0x{:x}; 0x{:x})",
    term.get_special_tag().0,
    term.get_special_value()
  )
}

pub unsafe fn format_cons(term: Term, f: &mut fmt::Formatter) -> fmt::Result {
  write!(f, "[")?;
  let mut first = true;

  if let Ok(Some(tail)) = cons::for_each(term, |elem| {
    if !first {
      write!(f, ", ").unwrap();
    } else {
      first = false;
    }
    write!(f, "{}", elem).unwrap();
    Ok(())
  }) {
    if tail != Term::nil() {
      // Improper list, show tail
      write!(f, "| {}", tail)?;
    }
  }

  write!(f, "]")
}

/// Depending on cargo.toml setting, print a fancy unicode or a simple quote
#[inline]
fn print_opening_quote(f: &mut fmt::Formatter) -> fmt::Result {
  if cfg!(feature = "fancy_string_quotes") {
    write!(f, "⹂")
  } else {
    write!(f, "\"")
  }
}

/// Depending on cargo.toml setting, print a fancy unicode or a simple quote
#[inline]
fn print_closing_quote(f: &mut fmt::Formatter) -> fmt::Result {
  if cfg!(feature = "fancy_string_quotes") {
    write!(f, "‟")
  } else {
    write!(f, "\"")
  }
}

pub unsafe fn format_cons_ascii(term: Term, f: &mut fmt::Formatter) -> fmt::Result {
  print_opening_quote(f)?;

  if let Ok(Some(tail)) = cons::for_each(term, |elem| {
    let ch = elem.get_small_unsigned();
    if !(cfg!(feature = "fancy_string_quotes")) && ch == 34 {
      write!(f, "\\\"").unwrap();
    } else {
      write!(f, "{}", ch as u8 as char).unwrap();
    }
    Ok(())
  }) {
    if tail != Term::nil() {
      panic!("Can't print improper list as ASCII, tail={}", tail)
    }
  }

  print_closing_quote(f)
}
