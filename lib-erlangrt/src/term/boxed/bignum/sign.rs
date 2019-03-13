#[derive(Debug, Clone)]
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
}
