use emulator::heap::Heap;
use fail::{Hopefully, Error};
use rt_defs::{storage_bytes_to_words, Word};
use term::boxed::{BoxHeader, BOXTYPETAG_BINARY};
use std::ptr;

pub enum BoxBinaryPayload {
  // contains size, followed in memory by the data bytes
  ProcBin { nbytes: Word } ,
  // contains reference to heapbin
  RefBin,
  // stores data on a separate heap somewhere else with refcount
  HeapBin { nbytes: Word, refc: Word },
}

/// Binary which stores everything in its allocated memory on process heap.
#[allow(dead_code)]
pub struct Binary {
  header: BoxHeader,
  pub payload: BoxBinaryPayload,
}

impl Binary {
  fn new(nbytes: usize) -> Binary {
    let arity = Binary::storage_size(nbytes) - 1;
    Binary {
      header: BoxHeader::new(BOXTYPETAG_BINARY, arity),
      payload: BoxBinaryPayload::ProcBin {nbytes}
    }
  }

  pub fn storage_size(nbytes: usize) -> usize {
    storage_bytes_to_words(std::mem::size_of::<Binary>())
        + storage_bytes_to_words(nbytes)
  }

  pub unsafe fn create_into(hp: &mut Heap, n_bytes: usize) -> Hopefully<*mut Binary>
  {
    let n_words = Binary::storage_size(n_bytes);
    let this = hp.alloc_words::<Binary>(n_words, false)?;

    ptr::write(this, Binary::new(n_bytes));

    Ok(this)
  }


  /// Given a byte array, copy it to the binary's memory (depending on
  /// the binary type).
  pub unsafe fn store(this: *mut Binary, data: &[u8]) -> Hopefully<()> {
    let data_len = data.len();
    if data_len == 0 {
      return Ok(())
    }

    match (*this).payload {
      BoxBinaryPayload::ProcBin {n} => {
        return Err(Error::ProcBinTooSmall(data_len, n))
      },
      BoxBinaryPayload::HeapBin {n, _refc} => {
        return Err(Error::HeapBinTooSmall(data_len, n))
      },
      BoxBinaryPayload::RefBin => {
        return Err(Error::CannotCopyIntoRefbin)
      },
    }

    // Take next word after the record end, that'll be first data word.
    let bin_bytes = this.offset(1) as *mut u8;
    ptr::copy_nonoverlapping(&data[0], bin_bytes, data_len);
    Ok(())
  }
}
