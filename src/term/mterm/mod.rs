//! MTerm is a memory representation of Term
pub mod mpid;

pub enum BoxType {
  Pid,
  Tuple,
  List,
}

/// Term header in memory, followed by corresponding data.
pub struct BoxHeader {
  pub t: BoxType
}

impl BoxHeader {
  pub fn new(t: BoxType) -> BoxHeader {
    BoxHeader { t }
  }
}
