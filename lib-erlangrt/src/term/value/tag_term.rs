// Structure of term:
// [ Value or a pointer ] [ TAG_* value 3 bits ]
//
pub const TERM_TAG_BITS: usize = 3;
pub const TERM_TAG_MASK: usize = (1 << TERM_TAG_BITS) - 1;

#[derive(Eq, PartialEq, Debug, Ord, PartialOrd)]
pub struct TermTag(pub usize);

impl TermTag {
  #[inline]
  pub const fn get(self) -> usize {
    self.0
  }
}

pub const TERMTAG_BOXED: TermTag = TermTag(0);
pub const TERMTAG_HEADER: TermTag = TermTag(1);
pub const TERMTAG_CONS: TermTag = TermTag(2);
// From here and below, values are immediate (fit into a single word)
pub const TERMTAG_SMALL: TermTag = TermTag(3);
pub const TERMTAG_ATOM: TermTag = TermTag(4);
pub const TERMTAG_LOCALPID: TermTag = TermTag(5);
pub const TERMTAG_LOCALPORT: TermTag = TermTag(6);
pub const TERMTAG_SPECIAL: TermTag = TermTag(7);
