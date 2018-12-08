use crate::{
  emulator::heap::Heap,
  fail::{Error, RtResult},
  rt_defs::{storage_bytes_to_words, Word, WordSize},
  term::boxed::{BoxHeader, BOXTYPETAG_BINARY},
};
use core::{fmt, ptr};

#[allow(dead_code)]
pub enum BinaryType {
  // contains size, followed in memory by the data bytes
  ProcBin { nbytes: Word },
  // contains reference to heapbin
  RefBin,
  // stores data on a separate heap somewhere else with refcount
  HeapBin { nbytes: Word, refc: Word },
}

/// Binary which stores everything in its allocated memory on process heap.
#[allow(dead_code)]
pub struct Binary {
  header: BoxHeader,
  pub bin_type: BinaryType,
}

impl Binary {
  fn new(nbytes: usize) -> Binary {
    let arity = Binary::storage_size(nbytes).words();
    Binary {
      header: BoxHeader::new(BOXTYPETAG_BINARY, arity),
      bin_type: BinaryType::ProcBin { nbytes },
    }
  }

  pub fn storage_size(nbytes: usize) -> WordSize {
    WordSize::new(
      storage_bytes_to_words(std::mem::size_of::<Binary>()).words()
        + storage_bytes_to_words(nbytes).words(),
    )
  }

  pub unsafe fn create_into(hp: &mut Heap, n_bytes: usize) -> RtResult<*mut Binary> {
    let n_words = Binary::storage_size(n_bytes);
    let this = hp.alloc::<Binary>(n_words, false)?;

    ptr::write(this, Binary::new(n_bytes));

    Ok(this)
  }


  /// Given a byte array, copy it to the binary's memory (depending on
  /// the binary type).
  pub unsafe fn store(this: *mut Binary, data: &[u8]) -> RtResult<()> {
    let data_len = data.len();
    if data_len == 0 {
      return Ok(());
    }

    match (*this).bin_type {
      BinaryType::ProcBin { nbytes: n } => {
        if n < data_len {
          return Err(Error::ProcBinTooSmall(data_len, n));
        }
      }
      BinaryType::HeapBin { nbytes: n, refc: _ } => {
        if n < data_len {
          return Err(Error::HeapBinTooSmall(data_len, n));
        }
      }
      BinaryType::RefBin => return Err(Error::CannotCopyIntoRefbin),
    }

    // Take a byte after the Binary struct, that'll be first data byte
    let bin_bytes = this.add(1) as *mut u8;

    ptr::copy_nonoverlapping(&data[0], bin_bytes, data_len);
    Ok(())
  }


  #[inline]
  unsafe fn get_byte(this: *const Binary, i: usize) -> u8 {
    let p = this.add(1) as *const u8;
    core::ptr::read(p.add(i))
  }


  /// Called from LTerm formatting function to print binary contents
  pub unsafe fn format_binary(
    this: *const Binary,
    f: &mut fmt::Formatter,
  ) -> fmt::Result {
    write!(f, "<<");
    match (*this).bin_type {
      BinaryType::RefBin => {
        write!(f, "#refbin");
      }
      BinaryType::ProcBin { nbytes } => {
        write!(f, "[size={}]", nbytes);
        for i in 0..nbytes {
          if i > 0 {
            write!(f, ", ");
          }
          write!(f, "{}", Binary::get_byte(this, i));
        }
      }
      BinaryType::HeapBin { nbytes, refc: _ } => {
        write!(f, "#heapbin[size={}]", nbytes);
      }
    }
    write!(f, ">>")
  }
}
