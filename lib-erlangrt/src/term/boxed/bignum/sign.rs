use std::cmp::Ordering;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Sign {
  Positive,
  Negative,
  Zero,
}

impl Sign {
  #[allow(dead_code)]
  pub fn new(val: isize) -> Self {
    match val.cmp(&0) {
      Ordering::Less => Sign::Negative,
      Ordering::Equal => Sign::Zero,
      Ordering::Greater => Sign::Positive,
    }
  }

  /// Take sign out of the value, and return the sign and the positive value
  pub fn split(val: isize) -> (Self, usize) {
    match val.cmp(&0) {
      Ordering::Less => (Sign::Negative, -val as usize),
      Ordering::Equal => (Sign::Zero, 0),
      Ordering::Greater => (Sign::Positive, val as usize),
    }
  }
}
