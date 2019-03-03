use crate::{
  defs::{ByteSize, WordSize},
  emulator::heap::Heap,
  fail::{RtErr, RtResult},
  term::boxed::{
    binary::{
      binaryheap_bin::BinaryHeapBinary, procheap_bin::ProcessHeapBinary,
      refc_bin::ReferenceToBinary,
    },
    BoxHeader, BOXTYPETAG_BINARY,
  },
};
use core::{fmt, ptr};

pub mod b_match;
mod binaryheap_bin;
mod procheap_bin;
mod refc_bin;

pub struct BitSize {
  pub size: ByteSize,
  pub last_byte_bits: u8,
}

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
  /* Size is stored in the storage object, which is overlaid onto this depending
   * on the bin_type */
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

  pub unsafe fn create_into(hp: &mut Heap, size: ByteSize) -> RtResult<*mut Binary> {
    if size.bytes() == 0 {
      // Return binary {} immediate special instead!
      return Err(RtErr::CreatingZeroSizedBinary);
    }
    let b_type = Self::get_binary_type_for(size);
    match b_type {
      BinaryType::ProcessHeap => ProcessHeapBinary::create_into(hp, size),
      BinaryType::BinaryHeap => panic!("notimpl! create binary on the binary heap"),
      BinaryType::RefToBinaryHeap => panic!("notimpl! create ref to binary heap"),
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
          return Err(RtErr::ProcBinTooSmall(data_len, avail_size));
        }
      }
      BinaryType::BinaryHeap => {
        let bh_ptr = this as *mut BinaryHeapBinary;
        let avail_size = (*bh_ptr).size.bytes();
        if (*bh_ptr).size.bytes() < data_len {
          return Err(RtErr::HeapBinTooSmall(data_len, avail_size));
        }
      }
      BinaryType::RefToBinaryHeap => {
        // TODO: Maybe should be possible? Assist with resolution into BinaryHeapBinary
        return Err(RtErr::CannotCopyIntoRefbin);
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

  unsafe fn generic_switch<T>(
    this: *const Binary,
    on_proc_bin: fn(*const ProcessHeapBinary) -> T,
    on_ref_bin: fn(*const ReferenceToBinary) -> T,
    on_binheap_bin: fn(*const BinaryHeapBinary) -> T,
  ) -> T {
    match (*this).bin_type {
      BinaryType::ProcessHeap => {
        on_proc_bin(this as *const ProcessHeapBinary)
      }
      BinaryType::RefToBinaryHeap => {
        on_ref_bin(this as *const ReferenceToBinary)
      }
      BinaryType::BinaryHeap => {
        on_binheap_bin(this as *const BinaryHeapBinary)
      }
    }
  }

  unsafe fn generic_switch_mut<T>(
    this: *mut Binary,
    on_proc_bin: fn(*mut ProcessHeapBinary) -> T,
    on_ref_bin: fn(*mut ReferenceToBinary) -> T,
    on_binheap_bin: fn(*mut BinaryHeapBinary) -> T,
  ) -> T {
    match (*this).bin_type {
      BinaryType::ProcessHeap => {
        on_proc_bin(this as *mut ProcessHeapBinary)
      }
      BinaryType::RefToBinaryHeap => {
        on_ref_bin(this as *mut ReferenceToBinary)
      }
      BinaryType::BinaryHeap => {
        on_binheap_bin(this as *mut BinaryHeapBinary)
      }
    }
  }

  /// Get the size in bytes of any type of binary.
  pub unsafe fn get_size(this: *const Binary) -> ByteSize {
    Self::generic_switch(this,
      |phb_ptr| {
        (*phb_ptr).size
      },
      |refb_ptr| {
        (*refb_ptr).size
      },
      |bhb_ptr| {
        (*bhb_ptr).size
      })
    }


  /// For any binary retrieve const data pointer and size
  pub unsafe fn get_data(this: *const Binary) -> *const u8 {
    match (*this).bin_type {
      BinaryType::ProcessHeap => {
        let phb_ptr = this as *const ProcessHeapBinary;
        phb_ptr.add(1) as *const u8
      }
      BinaryType::RefToBinaryHeap => {
        let refb_ptr = this as *const ReferenceToBinary;
        refb_ptr.add(1) as *const u8
      }
      BinaryType::BinaryHeap => {
        let bhb_ptr = this as *const BinaryHeapBinary;
        bhb_ptr.add(1) as *const u8
      }
    }
  }

  pub unsafe fn get_data_mut(this: *const Binary) -> *mut u8 {
    match (*this).bin_type {
      BinaryType::ProcessHeap => {
        let phb_ptr = this as *mut ProcessHeapBinary;
        phb_ptr.add(1) as *mut u8
      }
      BinaryType::RefToBinaryHeap => {
        let refb_ptr = this as *mut ReferenceToBinary;
        refb_ptr.add(1) as *mut u8
      }
      BinaryType::BinaryHeap => {
        let bhb_ptr = this as *mut BinaryHeapBinary;
        bhb_ptr.add(1) as *mut u8
      }
    }
  }

  /// Called from LTerm formatting function to print binary contents
  pub unsafe fn format_binary(
    this: *const Binary,
    f: &mut fmt::Formatter,
  ) -> fmt::Result {
    let datap = Self::get_data_mut(this);
    match (*this).bin_type {
      BinaryType::RefToBinaryHeap => {
        let refb_ptr = this as *mut ReferenceToBinary;
        let refb_size = (*refb_ptr).size;
        write!(f, "#refbin[{}]<<", refb_size)?;
        panic!("notimpl: printing refbin to binary heap");
      }
      BinaryType::ProcessHeap => {
        let phb_ptr = this as *mut ProcessHeapBinary;
        let phb_size = (*phb_ptr).size;
        write!(f, "#procbin[{}]<<", phb_size)?;
        for i in 0..phb_size.bytes() {
          if i > 0 {
            write!(f, ", ")?;
          }
          write!(f, "{}", core::ptr::read(datap.add(i)))?;
        }
      }
      BinaryType::BinaryHeap => {
        let bhb_ptr = this as *mut BinaryHeapBinary;
        let bhb_size = (*bhb_ptr).size;
        write!(f, "#heapbin[{}]<<", bhb_size)?;
        panic!("notimpl: printing bin on binary heap");
      }
    }
    write!(f, ">>")
  }
}
