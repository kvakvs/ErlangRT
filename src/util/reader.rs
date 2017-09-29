use std::fs::File;
use std::io::Read;

pub struct Reader {
  file: File,
}

impl Reader {
  pub fn new(filename: &str) -> Reader {
    let mut file = File::open(filename).unwrap();
//    let mut buf = [0u8; 12];
//    file.read(&mut buf).unwrap();
    Reader { file }
  }
}

