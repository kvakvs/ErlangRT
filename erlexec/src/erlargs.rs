use std::collections::HashMap;

#[derive(Debug)]
pub struct ErlArgs {
  dict: HashMap<String, Vec<String>>,
}

impl ErlArgs {
  pub fn new() -> Self {
    Self {
      dict: HashMap::new(),
    }
  }

  /// Copy args from any string iterator source
  pub fn populate_with<ITER>(&mut self, mut iter: ITER)
  where
    ITER: Iterator<Item = String>,
  {
    loop {
      if let Some(s) = iter.next() {
        println!("{}", s);
      } else {
        break;
      }
    }
  }
}
