use defs::Word;

pub struct Heap {
  data: Vec<Word>,
}

impl Heap {
  pub fn new() -> Heap {
    Heap{
      data: Vec::new(),
    }
  }
}
