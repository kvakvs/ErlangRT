extern crate bytes;

use std::fs::File;
use std::io;
use std::io::Read;
use std::path::{Path, PathBuf};

use rterror;

pub struct Reader {
  file: File,
}

impl Reader {
  pub fn new(filename: &PathBuf) -> Reader {
    let mut file = File::open(filename).unwrap();
    Reader { file }
  }

  pub fn ensure_bytes(&mut self, sample: &bytes::Bytes) -> Result<(), rterror::Error> {
    let mut file = &self.file;
    //let mut take = file.take(b.len() as u64);
    //let mut buf = String::new();
    let mut actual = bytes::BytesMut::with_capacity(sample.len());
    file.read_exact(&mut actual);

    let b1 = actual.as_ref();
    let b2 = sample.as_ref();
    if b1 == b2 { return Ok(()) }
//    match take.read_to_string(&mut buf) {
//      Ok(_) => if b.compare(buf) { return Ok(()) },
//      Err(_e) => {}
//    }
    let msg = format!("Expected: {:?}", sample);
    Err(rterror::Error::CodeLoadingFailed(msg))
  }
}

