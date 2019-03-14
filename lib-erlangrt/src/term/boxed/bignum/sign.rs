#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Sign {
  Positive,
  Negative,
  Zero,
}

impl Sign {
  pub fn new(val: isize) -> Self {
    if val == 0 {
      Sign::Zero
    } else if val > 0 {
      Sign::Positive
    } else {
      Sign::Negative
    }
  }

  /// Take sign out of the value, and return the sign and the positive value
  pub fn split(val: isize) -> (Self, usize) {
    if val == 0 {
      (Sign::Zero, 0)
    } else if val > 0 {
      (Sign::Positive, val as usize)
    } else {
      (Sign::Negative, -val as usize)
    }
  }
}
