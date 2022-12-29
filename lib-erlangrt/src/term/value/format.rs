//! Implement format trait (Display) for Term
// Printing low_level Terms as "{}"
use crate::{
  defs::Word,
  emulator::atom,
  term::{
    boxed::{self, box_header::BoxHeader, boxtype},
    cons, PrimaryTag, SpecialLoadtime, SpecialReg, SpecialTag, Term,
  },
};
use core::fmt;

impl fmt::Display for Term {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    if self.is_non_value() {
      return write!(f, "#Nonvalue<>");
    }
    match self.get_term_tag() {
      PrimaryTag::BOX_PTR => unsafe {
        if self.is_cp() {
          write!(f, "#Cp<{:p}>", self.get_cp_ptr::<Word>())
        } else {
          let box_ptr = self.get_box_ptr::<boxed::BoxHeader>();
          // `p` can't be null because non_value=0 is checked above
          format_box_contents(box_ptr, f)
        }
      },

      PrimaryTag::CONS_PTR => unsafe {
        if self.cons_is_ascii_string() {
          format_cons_ascii(*self, f)
        } else {
          format_cons(*self, f)
        }
      },

      PrimaryTag::SMALL_INT => write!(f, "{}", self.get_small_signed()),

      PrimaryTag::SPECIAL => format_special(*self, f),

      PrimaryTag::LOCAL_PID => write!(f, "#Pid<{}>", self.get_term_val_without_tag()),

      PrimaryTag::LOCAL_PORT => write!(f, "#Port<{}>", self.get_term_val_without_tag()),

      PrimaryTag::ATOM => match atom::to_str(*self) {
        Ok(s) => {
          if atom::is_printable_atom(&s) {
            write!(f, "{s}")
          } else {
            write!(f, "'{s}'")
          }
        }
        Err(e) => write!(f, "#Atom<printing failed {e:?}>"),
      },

      PrimaryTag::HEADER => {
        let h = boxed::BoxHeader::headerword_to_storage_size(self.raw());
        write!(f, "#Header({h})")
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
    boxtype::BOXTYPETAG_BIGINTEGER => write!(f, "#Big<>"),
    boxtype::BOXTYPETAG_TUPLE => {
      let tuple_ptr = trait_ptr as *const boxed::Tuple;
      (*tuple_ptr).format(f)
    }
    boxtype::BOXTYPETAG_CLOSURE => {
      let fun_p = trait_ptr as *const boxed::Closure;
      write!(f, "#Fun<{}>", (*fun_p).mfa)
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
      write!(f, "#Import<{}>", (*iptr).mfarity)
    }
    boxtype::BOXTYPETAG_EXPORT => {
      let eptr = trait_ptr as *const boxed::Export;
      write!(f, "#Export<{}>", (*eptr).exp.mfa)
    }
    boxtype::BOXTYPETAG_BINARY_MATCH_STATE => write!(f, "BinaryMatchState<>"),
    boxtype::BOXTYPETAG_JUMP_TABLE => {
      let jptr = trait_ptr as *const boxed::JumpTable;
      (*jptr).format(f)
    }
    _ => panic!("Unexpected header tag {:?}", box_type),
  }
}

// Formatting helpers
//

fn format_special(term: Term, f: &mut fmt::Formatter) -> fmt::Result {
  match term.get_special_tag() {
    SpecialTag::CONST => {
      if term == Term::nil() {
        return write!(f, "[]");
      } else if term.is_non_value() {
        panic!("This must have been handled above in call stack");
        // return write!(f, "#Nonvalue<>");
      } else if term == Term::empty_binary() {
        return write!(f, "<<>>");
      } else if term == Term::empty_tuple() {
        return write!(f, "{{}}");
      }
    }
    SpecialTag::REG => {
      let r_tag = term.get_reg_tag();
      let r_val = term.get_reg_value();
      if r_tag == SpecialReg::REG_X {
        return write!(f, "#x<{r_val}>");
      } else if r_tag == SpecialReg::REG_Y {
        return write!(f, "#y<{r_val}>");
      } else if r_tag == SpecialReg::REG_FLOAT {
        return write!(f, "#f<{r_val}>");
      } else {
        panic!("Unknown special reg tag {:?}", r_tag)
      }
    }
    SpecialTag::OPCODE => return write!(f, "Opcode({})", term.get_opcode_value()),
    SpecialTag::CATCH => return write!(f, "Catch({:p})", term.get_catch_ptr()),
    SpecialTag::LOAD_TIME => {
      let lt_tag = term.get_loadtime_tag();
      let lt_val = term.get_loadtime_val();
      if lt_tag == SpecialLoadtime::ATOM {
        return write!(f, "LtAtom({lt_val})");
      } else if lt_tag == SpecialLoadtime::LABEL {
        return write!(f, "LtLabel({lt_val})");
      } else if lt_tag == SpecialLoadtime::LITERAL {
        return write!(f, "LtLit({lt_val})");
      } else {
        panic!("Unknown special loadtime tag {:?}", lt_tag)
      }
    }
    _ => {}
  }
  write!(
    f,
    "#Special({}; 0x{:x})",
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
    write!(f, "{elem}").unwrap();
    Ok(())
  }) {
    if tail != Term::nil() {
      // Improper list, show tail
      write!(f, "| {tail}")?;
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

    #[allow(clippy::overly_complex_bool_expr)]
    #[allow(clippy::nonminimal_bool)]
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
