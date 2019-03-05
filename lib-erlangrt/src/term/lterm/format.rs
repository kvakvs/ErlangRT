//! Implement format trait (Display) for LTerm
// Printing low_level Terms as "{}"
use crate::{
  defs::Word,
  emulator::atom,
  term::{
    boxed,
    lterm::{cons, lterm_impl::*},
  },
};
use core::fmt;

impl fmt::Display for LTerm {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    if self.is_non_value() {
      return write!(f, "NON_VALUE");
    }
    match self.get_term_tag() {
      TERMTAG_BOXED => unsafe {
        if self.is_cp() {
          write!(f, "CP({:p})", self.get_cp_ptr::<Word>())
        } else {
          let p = self.get_box_ptr::<LTerm>();
          // `p` can't be null because non_value=0 is checked above
          format_box_contents(*p, p as *const Word, f)
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
  value_at_ptr: LTerm,
  val_ptr: *const Word,
  f: &mut fmt::Formatter,
) -> fmt::Result {
  let h_tag = boxed::headerword_to_boxtype(value_at_ptr.raw());
  match h_tag {
    boxed::BOXTYPETAG_BINARY => {
      let trait_ptr = boxed::Binary::get_trait(val_ptr as *const boxed::Binary);
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
    boxed::BOXTYPETAG_BIGINTEGER => write!(f, "Big<>"),
    boxed::BOXTYPETAG_TUPLE => format_tuple(val_ptr, f),
    boxed::BOXTYPETAG_CLOSURE => {
      let fun_p = val_ptr as *const boxed::Closure;
      write!(f, "Fun<{}>", (*fun_p).mfa)
    }
    boxed::BOXTYPETAG_FLOAT => {
      let fptr = val_ptr as *const boxed::Float;
      write!(f, "{}", (*fptr).value)
    }
    boxed::BOXTYPETAG_EXTERNALPID => write!(f, "ExtPid<>"),
    boxed::BOXTYPETAG_EXTERNALPORT => write!(f, "ExtPort<>"),
    boxed::BOXTYPETAG_EXTERNALREF => write!(f, "ExtRef<>"),
    boxed::BOXTYPETAG_IMPORT => {
      let iptr = val_ptr as *const boxed::Import;
      write!(f, "Import<{}>", (*iptr).mfarity)
    }
    boxed::BOXTYPETAG_EXPORT => {
      let eptr = val_ptr as *const boxed::Export;
      write!(f, "Export<{}>", (*eptr).exp.mfa)
    }
    boxed::BOXTYPETAG_BINARY_MATCH_STATE => write!(f, "BinaryMatchState<>"),

    _ => panic!("Unexpected header tag {:?}", h_tag),
  }
}

// Formatting helpers
//

fn format_special(term: LTerm, f: &mut fmt::Formatter) -> fmt::Result {
  match term.get_special_tag() {
    SPECIALTAG_CONST => {
      if term == LTerm::nil() {
        return write!(f, "[]");
      } else if term.is_non_value() {
        return write!(f, "NON_VALUE");
      } else if term == LTerm::empty_binary() {
        return write!(f, "<<>>");
      } else if term == LTerm::empty_tuple() {
        return write!(f, "{{}}");
      }
    }
    SPECIALTAG_REGX => return write!(f, "X{}", term.get_special_value()),
    SPECIALTAG_REGY => return write!(f, "Y{}", term.get_special_value()),
    SPECIALTAG_REGFP => return write!(f, "F{}", term.get_special_value()),
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

/// Given `p`, a pointer to tuple header word, format tuple contents.
unsafe fn format_tuple(p: *const Word, f: &mut fmt::Formatter) -> fmt::Result {
  let tptr = match boxed::Tuple::from_pointer(p) {
    Ok(x) => x,
    Err(e) => return write!(f, "<err formatting tuple: {:?}>", e),
  };

  write!(f, "{{")?;

  let arity = (*tptr).get_arity();
  for i in 0..arity {
    write!(f, "{}", boxed::Tuple::get_element_base0(tptr, i))?;
    if i < arity - 1 {
      write!(f, ", ")?
    }
  }
  write!(f, "}}")
}

pub unsafe fn format_cons(term: LTerm, f: &mut fmt::Formatter) -> fmt::Result {
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
    if tail != LTerm::nil() {
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

pub unsafe fn format_cons_ascii(term: LTerm, f: &mut fmt::Formatter) -> fmt::Result {
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
    if tail != LTerm::nil() {
      panic!("Can't print improper list as ASCII, tail={}", tail)
    }
  }

  print_closing_quote(f)
}
