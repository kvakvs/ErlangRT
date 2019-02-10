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
  /// contains size, followed in memory by the actual data bytes.
  /// The pointer to `Binary` should be converted into `ProcessHeapBinary`.
  ProcessHeap,
  /// contains reference to heapbin, the pointer to `Binary`
  /// should be converted into `ReferenceToBinary`.
  RefToBinaryHeap,
  /// stores data on a separate heap with refcount away from the process,
  /// the pointer to `Binary` should be converted into `BinaryHeapBinary`.
  BinaryHeap,
}

/// Binary which stores everything in its allocated memory on process heap.
#[allow(dead_code)]
pub struct Binary {
  header: BoxHeader,
  /// Based on the bin_type, the pointer should be converted to one of binary
  /// subtypes and then used accordingly.
  bin_type: BinaryType,
  // Size is stored in the storage object, which is overlaid onto this depending
  // on the bin_type
}

impl Binary {
  fn get_binary_type_for(size: ByteSize) -> BinaryType {
    if size.bytes() <= ProcessHeapBinary::ONHEAP_THRESHOLD {
      return BinaryType::ProcessHeap;
    }
    BinaryType::BinaryHeap
  }

  fn new(bin_type: BinaryType, arity: WordSize) -> Binary {
    Binary {
      header: BoxHeader::new(BOXTYPETAG_BINARY, arity.words()),
      bin_type,
    }
  }

//  pub fn storage_size(b_type: BinaryType, size: ByteSize) -> WordSize {
//    let header_size: ByteSize;
//    match b_type {
//      BinaryType::BinaryHeap => {
//        header_size = ByteSize::new(std::mem::size_of::<BinaryHeapBinary>());
//      }
//      BinaryType::ProcessHeap => {
//        header_size = ByteSize::new(std::mem::size_of::<ProcessHeapBinary>());
//      }
//      BinaryType::RefToBinaryHeap => {
//        header_size = ByteSize::new(std::mem::size_of::<ReferenceToBinary>());
//      }
//    }
//    WordSize::new(
//      header_size.words_rounded_up().words() + size.words_rounded_up().words(),
//    )
//  }

  pub unsafe fn create_into(hp: &mut Heap, size: ByteSize) -> RtResult<*mut Binary> {
    let b_type = Self::get_binary_type_for(size);
    match b_type {
      BinaryType::ProcessHeap => {
        ProcessHeapBinary::create_into(hp, size)
      },
      BinaryType::BinaryHeap => {
        panic!("notimpl! create binary on the binary heap")
      },
      BinaryType::RefToBinaryHeap => {
        panic!("notimpl! create ref to binary heap")
      },
    }
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
