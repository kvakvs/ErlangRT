use bytes::Bytes;
use std::path::PathBuf;

use rterror;
use util::reader;

pub struct Loader {}

impl Loader {
  pub fn new() -> Loader {
    Loader {}
  }

  /// Loading the module. Stage 1 validating the header and creating the loader state.
  pub fn load(&mut self, fname: &PathBuf) -> Result<(), rterror::Error> {
    let mut r = reader::Reader::new(fname);
    // Err(rterror::Error::FileNotFound("found".to_string()))
    let hdr_bytes = Bytes::from(&b"FOR@"[..]);
    r.ensure_bytes(&hdr_bytes);
    Ok(())
  }
}
