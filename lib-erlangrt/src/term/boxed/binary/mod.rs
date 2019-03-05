use core::fmt;

use crate::{
  defs::{self, data_reader::TDataReader, BitSize, WordSize},
  emulator::heap::Heap,
  fail::{RtErr, RtResult},
  term::{
    boxed::{
      binary::{
        binaryheap_bin::BinaryHeapBinary, procheap_bin::ProcessHeapBinary,
        refc_bin::ReferenceToBinary, slice::BinarySlice, trait_interface::TBinary,
      },
      boxtype::{self, BoxType},
      trait_interface::TBoxed,
      BoxHeader,
    },
    classify,
    lterm::LTerm,
  },
};

pub mod binaryheap_bin;
pub mod match_state;
pub mod procheap_bin;
pub mod refc_bin;
pub mod slice;
pub mod trait_interface;

// pub use self::{match_state::*, bitsize::*, slice::*, trait_interface::*};

#[derive(Debug, Copy, Clone)]
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
  /// Subbinary, points to a fragment of another binary. A slice, literally.
  Slice,
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

impl TBoxed for Binary {
  fn get_class(&self) -> classify::TermClass {
    classify::CLASS_BINARY
  }

  fn get_type(&self) -> BoxType {
    boxtype::BOXTYPETAG_BINARY
  }
}

impl Binary {
  fn get_binary_type_for_creation(size: BitSize) -> BinaryType {
    if size.get_byte_size_rounded_up().bytes() <= ProcessHeapBinary::ONHEAP_THRESHOLD {
      return BinaryType::ProcessHeap;
    }
    BinaryType::BinaryHeap
  }

  fn new(bin_type: BinaryType, arity: WordSize) -> Binary {
    Binary {
      header: BoxHeader::new::<Binary>(arity.words()),
      bin_type,
    }
  }

  pub unsafe fn create_into(size: BitSize, hp: &mut Heap) -> RtResult<*mut TBinary> {
    if size.bit_count == 0 {
      // Return binary {} immediate special instead!
      return Err(RtErr::CreatingZeroSizedBinary);
    }
    let b_type = Self::get_binary_type_for_creation(size);
    match b_type {
      BinaryType::ProcessHeap => ProcessHeapBinary::create_into(size, hp),
      BinaryType::BinaryHeap => unimplemented!("create binary on the binary heap"),
      BinaryType::RefToBinaryHeap => unimplemented!("create ref to binary heap"),
      BinaryType::Slice => panic!("Can't create slice here"),
    }
  }

  //  #[inline]
  //  unsafe fn get_byte(this: *const Binary, i: usize) -> u8 {
  //    unimplemented!();
  //    // let p = this.add(1) as *const u8;
  //    // core::ptr::read(p.add(i))
  //  }

  unsafe fn generic_switch<T>(
    this: *const Binary,
    on_proc_bin: fn(*const ProcessHeapBinary) -> T,
    on_ref_bin: fn(*const ReferenceToBinary) -> T,
    on_binheap_bin: fn(*const BinaryHeapBinary) -> T,
    on_slice_bin: fn(*const BinarySlice) -> T,
  ) -> T {
    match (*this).bin_type {
      BinaryType::ProcessHeap => on_proc_bin(this as *const ProcessHeapBinary),
      BinaryType::RefToBinaryHeap => on_ref_bin(this as *const ReferenceToBinary),
      BinaryType::BinaryHeap => on_binheap_bin(this as *const BinaryHeapBinary),
      BinaryType::Slice => on_slice_bin(this as *const BinarySlice),
    }
  }

  #[allow(dead_code)]
  unsafe fn generic_switch_mut<T>(
    this: *mut Binary,
    on_proc_bin: fn(*mut ProcessHeapBinary) -> T,
    on_ref_bin: fn(*mut ReferenceToBinary) -> T,
    on_binheap_bin: fn(*mut BinaryHeapBinary) -> T,
    on_slice_bin: fn(*mut BinarySlice) -> T,
  ) -> T {
    match (*this).bin_type {
      BinaryType::ProcessHeap => on_proc_bin(this as *mut ProcessHeapBinary),
      BinaryType::RefToBinaryHeap => on_ref_bin(this as *mut ReferenceToBinary),
      BinaryType::BinaryHeap => on_binheap_bin(this as *mut BinaryHeapBinary),
      BinaryType::Slice => on_slice_bin(this as *mut BinarySlice),
    }
  }

  pub unsafe fn get_trait(this: *const Binary) -> *const TBinary {
    Self::generic_switch(
      this,
      |phb_ptr| phb_ptr as *const TBinary,
      |refb_ptr| refb_ptr as *const TBinary,
      |bhb_ptr| bhb_ptr as *const TBinary,
      |slice_ptr| slice_ptr as *const TBinary,
    )
  }

  #[allow(dead_code)]
  pub unsafe fn get_trait_mut(this: *mut Binary) -> *mut TBinary {
    Self::generic_switch_mut(
      this,
      |phb_ptr| phb_ptr as *mut TBinary,
      |refb_ptr| refb_ptr as *mut TBinary,
      |bhb_ptr| bhb_ptr as *mut TBinary,
      |slice_ptr| slice_ptr as *mut TBinary,
    )
  }

  /// Convert a VM term representation into a dynamic dispatch Rust trait
  pub unsafe fn get_trait_from_term(t: LTerm) -> *const TBinary {
    let bin_p = t.get_box_ptr::<Binary>();
    Self::generic_switch(
      bin_p,
      |phb_ptr| phb_ptr as *const TBinary,
      |refb_ptr| refb_ptr as *const TBinary,
      |bhb_ptr| bhb_ptr as *const TBinary,
      |slice_ptr| slice_ptr as *const TBinary,
    )
  }

  pub unsafe fn format(bin_trait: *const TBinary, f: &mut fmt::Formatter) -> fmt::Result {
    // let size = (*bin_trait).get_bit_size();

    // Print opening '<<'
    write!(f, "<<")?;

    match (*bin_trait).get_byte_reader() {
      Some(byte_reader) => Self::format_binary_with(byte_reader, f)?,
      None => {
        let bit_reader = (*bin_trait).get_bit_reader();
        Self::format_binary_with(bit_reader, f)?
      }
    }

    write!(f, ">>")
  }

  fn format_binary_with<Reader>(reader: Reader, f: &mut fmt::Formatter) -> fmt::Result
  where
    Reader: TDataReader,
  {
    let size = reader.get_bit_size();
    let n_bytes = size.get_byte_size_rounded_down().bytes();

    // Print comma separated full bytes
    for i in 0..n_bytes {
      if i > 0 {
        write!(f, ", ").unwrap();
      }
      let b = reader.read(i);
      write!(f, "{}", b).unwrap();
    }

    // If last byte bits are not 0, print comma again and print the last byte
    let lbb = size.get_last_byte_bits();
    if lbb != 0 {
      if size.bit_count > defs::BYTE_BITS {
        write!(f, ", ")?;
      }
      let last_byte = reader.read(n_bytes);
      write!(f, "{}:{}", last_byte, lbb)?;
    }
    Ok(())
  }
}
