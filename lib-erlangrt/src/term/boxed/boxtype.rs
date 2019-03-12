// Structure of a header word:
// [ Arity ... ] [ Header type: 3 bits ] [ Header tag: 3 bits ]
//

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct BoxType(usize);

impl BoxType {
  #[inline]
  pub fn get(self) -> usize {
    self.0
  }
}

pub const BOXTYPETAG_TUPLE: BoxType = BoxType(10);
pub const BOXTYPETAG_BIGINTEGER: BoxType = BoxType(20); // todo: separate tag for negative?
pub const BOXTYPETAG_EXTERNALPID: BoxType = BoxType(30);
pub const BOXTYPETAG_EXTERNALREF: BoxType = BoxType(40);
pub const BOXTYPETAG_EXTERNALPORT: BoxType = BoxType(50);

// A function object with frozen (captured) variable values
pub const BOXTYPETAG_CLOSURE: BoxType = BoxType(60);

pub const BOXTYPETAG_FLOAT: BoxType = BoxType(70);
pub const BOXTYPETAG_IMPORT: BoxType = BoxType(80);
pub const BOXTYPETAG_EXPORT: BoxType = BoxType(90);
pub const BOXTYPETAG_MAP: BoxType = BoxType(100);

pub const BOXTYPETAG_BINARY: BoxType = BoxType(110);
pub const BOXTYPETAG_BINARY_MATCH_STATE: BoxType = BoxType(120);
pub const BOXTYPETAG_JUMP_TABLE: BoxType = BoxType(130);
// unused 13
// unused 14
// unused 15 => max 15 (1 << BOXTYPE_TAG_BITS)

// pub const BOXTYPE_TAG_BITS: usize = 4;

//#[allow(dead_code)]
// pub const BOXTYPE_TAG_MASK: usize = (1 << BOXTYPE_TAG_BITS) - 1;
