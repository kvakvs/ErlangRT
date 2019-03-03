use crate::{
  defs::{ByteSize, WordSize},
  emulator::heap::Heap,
  fail::{RtErr, RtResult},
  term::{
    boxed::{
      binary::{
        binaryheap_bin::BinaryHeapBinary, procheap_bin::ProcessHeapBinary,
        refc_bin::ReferenceToBinary, slice::BinarySlice, trait_interface::TBinary,
      },
      BoxHeader, BOXTYPETAG_BINARY,
    },
    lterm::LTerm,
  },
};

pub mod b_match;
mod binaryheap_bin;
mod procheap_bin;
mod refc_bin;
mod slice;
pub mod trait_interface;

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

impl Binary {
  fn get_binary_type_for_creation(size: ByteSize) -> BinaryType {
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

  pub unsafe fn create_into(hp: &mut Heap, size: ByteSize) -> RtResult<*mut TBinary> {
    if size.bytes() == 0 {
      // Return binary {} immediate special instead!
      return Err(RtErr::CreatingZeroSizedBinary);
    }
    let b_type = Self::get_binary_type_for_creation(size);
    match b_type {
      BinaryType::ProcessHeap => ProcessHeapBinary::create_into(hp, size),
      BinaryType::BinaryHeap => panic!("notimpl! create binary on the binary heap"),
      BinaryType::RefToBinaryHeap => panic!("notimpl! create ref to binary heap"),
      BinaryType::Slice => panic!("Can't create slice here"),
    }
  }

//  #[inline]
//  unsafe fn get_byte(this: *const Binary, i: usize) -> u8 {
//    panic!("notimpl");
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

  //  /// For any binary retrieve const data pointer and size
  //  pub unsafe fn get_data(this: *const Binary) -> *const u8 {
  //    Self::generic_switch(
  //      this,
  //      |phb_ptr| phb_ptr.add(1) as *const u8,
  //      |refb_ptr| refb_ptr.add(1) as *const u8,
  //      |bhb_ptr| bhb_ptr.add(1) as *const u8,
  //    )
  //  }

  //  pub unsafe fn get_data_mut(this: *mut Binary) -> *mut u8 {
  //    Self::generic_switch_mut(
  //      this,
  //      |phb_ptr| phb_ptr.add(1) as *mut u8,
  //      |refb_ptr| refb_ptr.add(1) as *mut u8,
  //      |bhb_ptr| bhb_ptr.add(1) as *mut u8,
  //    )
  //  }
}
