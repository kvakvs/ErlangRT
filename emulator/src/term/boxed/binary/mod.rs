use crate::{
  defs::{ByteSize, WordSize},
  emulator::heap::Heap,
  fail::{Error, RtResult},
  term::boxed::{
    binary::{
      binaryheap_bin::BinaryHeapBinary, procheap_bin::ProcessHeapBinary,
      refc_bin::ReferenceToBinary,
    },
    BoxHeader, BOXTYPETAG_BINARY,
  },
};
use core::{fmt, ptr};

mod binaryheap_bin;
mod procheap_bin;
mod refc_bin;

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub enum BinaryType {
  // contains size, followed in memory by the data bytes
  ProcessHeap,
  // contains reference to heapbin
  RefToBinaryHeap,
  // stores data on a separate heap somewhere else with refcount
  BinaryHeap,
}

/// Binary which stores everything in its allocated memory on process heap.
#[allow(dead_code)]
pub struct Binary {
  header: BoxHeader,
  bin_type: BinaryType,
}

impl Binary {
  fn get_binary_type_for(size: ByteSize) -> BinaryType {
    if size.bytes() <= ProcessHeapBinary::ONHEAP_THRESHOLD {
      return BinaryType::ProcessHeap;
    }
    BinaryType::BinaryHeap
  }

  fn new(b_type: BinaryType, size: ByteSize) -> Binary {
    let arity = Binary::storage_size(b_type, size).words();
    Binary {
      header: BoxHeader::new(BOXTYPETAG_BINARY, arity),
      bin_type: BinaryType::ProcessHeap,
    }
  }

  pub fn storage_size(b_type: BinaryType, size: ByteSize) -> WordSize {
    let header_size: ByteSize;
    match b_type {
      BinaryType::BinaryHeap => {
        header_size = ByteSize::new(std::mem::size_of::<BinaryHeapBinary>());
      }
      BinaryType::ProcessHeap => {
        header_size = ByteSize::new(std::mem::size_of::<ProcessHeapBinary>());
      }
      BinaryType::RefToBinaryHeap => {
        header_size = ByteSize::new(std::mem::size_of::<ReferenceToBinary>());
      }
    }
    WordSize::new(
      header_size.words_rounded_up().words() + size.words_rounded_up().words(),
    )
  }

  pub unsafe fn create_into(hp: &mut Heap, size: ByteSize) -> RtResult<*mut Binary> {
    let b_type = Binary::get_binary_type_for(size);
    let storage_sz = Binary::storage_size(b_type, size);
    let this = hp.alloc::<Binary>(storage_sz, false)?;

    ptr::write(this, Binary::new(b_type, size));

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
      BinaryType::ProcessHeap => {
        let phb_ptr = this as *mut ProcessHeapBinary;
        let avail_size = (*phb_ptr).size.bytes();
        if avail_size < data_len {
          return Err(Error::ProcBinTooSmall(data_len, avail_size));
        }
      }
      BinaryType::BinaryHeap => {
        let bh_ptr = this as *mut BinaryHeapBinary;
        let avail_size = (*bh_ptr).size.bytes();
        if (*bh_ptr).size.bytes() < data_len {
          return Err(Error::HeapBinTooSmall(data_len, avail_size));
        }
      }
      BinaryType::RefToBinaryHeap => {
        // TODO: Maybe should be possible? Assist with resolution into BinaryHeapBinary
        return Err(Error::CannotCopyIntoRefbin);
      }
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
    write!(f, "<<")?;
    match (*this).bin_type {
      BinaryType::RefToBinaryHeap => {
        let refb_ptr = this as *mut ReferenceToBinary;
        let refb_size = (*refb_ptr).size;
        write!(f, "#refbin[{}]", refb_size)?;
      }
      BinaryType::ProcessHeap => {
        let phb_ptr = this as *mut ProcessHeapBinary;
        let phb_size = (*phb_ptr).size;
        write!(f, "#procbin[{}]", phb_size)?;
        for i in 0..phb_size.bytes() {
          if i > 0 {
            write!(f, ", ")?;
          }
          write!(f, "{}", Binary::get_byte(this, i))?;
        }
      }
      BinaryType::BinaryHeap => {
        let bhb_ptr = this as *mut BinaryHeapBinary;
        let bhb_size = (*bhb_ptr).size;
        write!(f, "#heapbin[{}]", bhb_size)?;
      }
    }
    write!(f, ">>")
  }
}
