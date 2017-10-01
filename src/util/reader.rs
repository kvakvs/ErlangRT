extern crate bytes;

use bytes::{ByteOrder, BigEndian};
use std::str;
use std::fs::File;
use std::io;
use std::io::{Read, Seek, SeekFrom};
use std::path::{Path, PathBuf};
use std::cmp::min;

use types::Word;
use rterror;

fn module() -> &'static str { "File reader: " }

pub struct BinaryReader {
  buf: Vec<u8>,
  pos: usize,
}

impl BinaryReader {
  /// Open a binary file and read everything into buf.
  pub fn from_file(filename: &PathBuf) -> BinaryReader {
    let mut file = File::open(filename).unwrap();
    let mut buf: Vec<u8> = Vec::new();
    file.read_to_end(&mut buf);
    BinaryReader { buf, pos: 0 }
  }

  /// Just provide a premade buf. Used for testing.
  pub fn from_bytes(buf: Vec<u8>) -> BinaryReader {
    BinaryReader { buf, pos: 0 }
  }

  /// From the buffer take so many bytes as there are in `sample` and compare
  /// them.
  pub fn ensure_bytes(&mut self, sample: &bytes::Bytes)
                      -> Result<(), rterror::Error>
  {
    let actual = self.read_bytes(sample.len()).unwrap();

    let b2 = sample.as_ref();
    if actual.as_slice() == b2 { return Ok(()); }

    let msg = format!("{}Expected: {:?} actual {:?}",
                      module(), sample, actual);
    Err(rterror::Error::CodeLoadingFailed(msg))
  }

  /// From the buffer take 4 bytes and interpret them as big endian u32.
  pub fn read_u32be(&mut self) -> u32 {
    let r = bytes::BigEndian::read_u32(&self.buf[self.pos..self.pos+4]);
    self.pos += 4;
    r
//    let buf = self.read_bytes(4).unwrap();
//    ((buf[0] as u32) << 24)
//        | ((buf[1] as u32) << 16)
//        | ((buf[2] as u32) << 8)
//        | (buf[3] as u32)
  }

  /// Consume `size` bytes from `self.file` and return them as a `Vec<u8>`
  pub fn read_bytes(&mut self, size: Word) -> Result<Vec<u8>, rterror::Error> {
    if self.buf.len() < self.pos + size {
      // panic!("premature EOF");
      return Err(rterror::Error::CodeLoadingPrematureEOF);
    }

    let r = Vec::from(&self.buf[self.pos..self.pos + size]);
    self.pos += size;
    Ok(r)
  }

  /// Read `size` characters and return as a string
  pub fn read_str_utf8(&mut self, size: Word) -> Result<String, rterror::Error> {
    let buf = self.read_bytes(size)?;
    match str::from_utf8(&buf) {
      Ok(v) => Ok(v.to_string()),
      Err(e) => {
        let msg = format!("{}Invalid UTF-8 sequence: {}", module(), e);
        Err(rterror::Error::CodeLoadingFailed(msg))
      },
    }
  }

  /// Read `size` characters and return as a string
  pub fn read_str_latin1(&mut self, size: Word) -> Result<String, rterror::Error> {
    let buf = self.read_bytes(size)?;
    Ok(buf.iter().map(|&c| c as char).collect())
  }

  /// Read only 1 byte
  pub fn read_u8(&mut self) -> u8 {
    let r = self.buf[self.pos];
    self.pos += 1;
    r
  }

  /// Advance the position by `n` or till the end.
  pub fn skip(&mut self, n: Word) {
    self.pos = min(self.pos + n, self.buf.len() - 1);
  }
}
