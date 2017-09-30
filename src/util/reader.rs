extern crate bytes;

use std::str;
use std::fs::File;
use std::io;
use std::io::Read;
use std::path::{Path, PathBuf};

use types::Word;
use rterror;

fn module() -> &'static str { "File reader: " }

pub struct Reader {
  file: File,
}

impl Reader {
  /// Open a binary file for reading.
  pub fn new(filename: &PathBuf) -> Reader {
    let mut file = File::open(filename).unwrap();
    Reader { file }
  }

  /// From file read as many bytes as there are in `sample` and compare them.
  pub fn ensure_bytes(&mut self, sample: &bytes::Bytes)
                      -> Result<(), rterror::Error>
  {
    let actual = self.read_bytes(sample.len());

    let b2 = sample.as_ref();
    if actual.as_slice() == b2 { return Ok(()); }

    let msg = format!("{}Expected: {:?} actual {:?}",
                      module(), sample, actual);
    Err(rterror::Error::CodeLoadingFailed(msg))
  }

  /// From file read 4 bytes and interpret them as Big Endian u32.
  pub fn read_u32be(&mut self) -> u32 {
    let mut buf = self.read_bytes(4);
    ((buf[0] as u32) << 24)
        | ((buf[1] as u32) << 16)
        | ((buf[2] as u32) << 8)
        | (buf[3] as u32)
  }

  /// Consume `size` bytes from `self.file` and return them as a `Vec<u8>`
  pub fn read_bytes(&mut self, size: Word) -> Vec<u8> {
    let mut file = &self.file;
    let mut buf = Vec::with_capacity(size);
    file.take(size as u64).read_to_end(&mut buf).unwrap();
    buf
  }

  /// Read `size` characters and return as a string
  pub fn read_str(&mut self, size: Word) -> String {
    let buf = self.read_bytes(size);
    match str::from_utf8(&buf) {
      Ok(v) => v.to_string(),
      Err(e) => panic!("{}Invalid UTF-8 sequence: {}", module(), e),
    }
  }

  /// Read only 1 byte
  pub fn read_u8(&mut self) -> u8 {
    let mut file = &self.file;
    let mut b = [0u8; 1];
    file.read_exact(&mut b);
    b[0]
  }
}
