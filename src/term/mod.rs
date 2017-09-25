mod immediate;
mod primary_tag;

use types::Word;


#[derive(Copy, Clone)]
pub struct Term {
  value: Word
}


impl Term {
  // Get primary tag bits from a raw term
  pub fn primary_tag(&self) -> primary_tag::Tag {
    primary_tag::from_word(self.value)
  }

  pub fn is_imm(&self) -> bool {
    self.primary_tag() == primary_tag::Tag::Immediate
  }

  pub fn is_box(&self) -> bool {
    self.primary_tag() == primary_tag::Tag::Box
  }

  pub fn is_cons(&self) -> bool {
    self.primary_tag() == primary_tag::Tag::Cons
  }

  pub fn is_header(&self) -> bool {
    self.primary_tag() == primary_tag::Tag::Header
  }

  pub fn get_raw(&self) -> Word { self.value }

  //
  // Construction
  //

  pub fn new_raw(w: Word) -> Term {
    Term { value: w }
  }

  // From atom index create an atom. To create from string use vm::new_atom
  pub fn new_atom(index: Word) -> Term {
    Term { value: immediate::atom(index) }
  }
}
