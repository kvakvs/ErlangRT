use core::cmp::Ordering;

use crate::{
  defs::{Word, WordSize},
  emulator::heap::Heap,
  fail::RtResult,
  term::{
    boxed::{
      boxtype::{self, BoxType},
      trait_interface::TBoxed,
      BoxHeader,
    },
    classify,
    compare::cmp_terms,
    lterm::Term,
  },
};

enum MapType {
  FlatMap,
}

/// Map get result can either be a value `Found()` or the location in map
/// where the binary search collapsed to a zero interval, and missing element
/// can be inserted there in sorted order: `NotFoundAt()`.
pub enum MapGetResult {
  FoundAt(usize),
  ClosestLarger(usize),
}

/// Representation of Map on heap, either stored as a list of sorted pairs
/// or as a hash tree (HAMT).
/// TODO: implement HAMT, for now only using sorted list of pairs
pub struct Map {
  header: BoxHeader,
  map_type: MapType,
  /// Map headerword arity is the capacity, while this is the value count
  count: usize,
}

impl TBoxed for Map {
  fn get_class(&self) -> classify::TermClass {
    classify::CLASS_MAP
  }

  fn get_type(&self) -> BoxType {
    boxtype::BOXTYPETAG_MAP
  }
}

impl Map {
  /// Size of a tuple in memory with the header word (used for allocations)
  #[inline]
  pub fn storage_size(num_pairs: Word) -> WordSize {
    WordSize::new(2 * num_pairs) + BoxHeader::storage_size()
  }

  /// Capacity is how many extra words been allocated
  /// TODO: capacity is not words count on heap, but the count of k/v pairs
  fn new(num_pairs: usize) -> Self {
    let storage_size = Self::storage_size(num_pairs);
    Self {
      header: BoxHeader::new::<Map>(storage_size),
      map_type: MapType::FlatMap,
      count: 0,
    }
  }

  /// Returns allocated size used by this map on heap
  pub fn get_capacity(&self) -> usize {
    self.header.get_arity()
  }

  /// Returns actual element count, less or equal to the capacity
  pub fn get_count(&self) -> usize {
    self.count
  }

  /// Allocate `size+n` cells and form a Map in memory, return the pointer.
  pub fn create_into(hp: &mut Heap, num_pairs: Word) -> RtResult<*mut Map> {
    let n = Self::storage_size(num_pairs);
    let p = hp.alloc::<Map>(n, false)?;
    unsafe {
      core::ptr::write(p, Map::new(num_pairs));
    }
    Ok(p)
  }

  /// Add a key/value pair to map (unsorted).
  /// Note: the flatmap must be sorted for use
  pub unsafe fn add(this: *mut Map, key: Term, value: Term) -> RtResult<()> {
    // Take this+1 to get pointer after the struct end
    let p = this.add(1) as *mut Term;
    let insert_pos = match Self::get_internal(this, key)? {
      MapGetResult::FoundAt(pos) => pos,
      MapGetResult::ClosestLarger(pos) => {
        // Possible reallocation? For now just assert that it can grow
        assert!((*this).get_capacity() > (*this).get_count());

        // Shift elements one forward, each element is key and value pair
        core::ptr::copy(
          p.add(pos * 2),
          p.add(pos * 2 + 2),
          2 * ((*this).count - pos),
        );
        (*this).count += 1;

        pos
      }
    };
    // Write the key and value where they should go
    core::ptr::write(p.add(insert_pos * 2), key);
    core::ptr::write(p.add(insert_pos * 2 + 1), value);
    Ok(())
  }

  /// Find key in map
  #[allow(dead_code)]
  pub unsafe fn get(this: *const Map, key: Term) -> RtResult<Option<Term>> {
    // If found anything, return the value, otherwise not found
    match Self::get_internal(this, key)? {
      MapGetResult::FoundAt(i) => {
        let p = this.add(1) as *const Term;
        Ok(Some(core::ptr::read(p.add(i * 2 + 1))))
      }
      _ => Ok(None),
    }
  }

  #[inline]
  unsafe fn get_internal(this: *const Map, key: Term) -> RtResult<MapGetResult> {
    match (*this).map_type {
      MapType::FlatMap => Self::get_flatmap(this, key),
    }
  }

  unsafe fn get_flatmap(this: *const Map, key: Term) -> RtResult<MapGetResult> {
    // Take this+1 to get pointer after the struct end
    let p = this.add(1) as *mut Term;
    let count = (*this).get_count();

    // Binary search goes here
    // Assuming: Keys are sorted in ascending order
    println!("map:get size={} key={}", count, key);

    if count == 0 {
      return Ok(MapGetResult::ClosestLarger(0));
    }

    let mut a = 0usize;
    let mut b = count - 1;
    loop {
      let median = a + (b - a) / 2;
      let median_value = core::ptr::read(p.add(median * 2));
      // The key is less than median, step left
      println!("map:get a={} b={} median={}", a, b, median);
      match cmp_terms(median_value, key, true)? {
        Ordering::Less => {
          if a == b {
            // search cannot shrink when the range is already length 0
            // Suggest to the caller that we've found where the closest larger
            // element is located for possible insertion.
            return Ok(MapGetResult::ClosestLarger(a + 1));
          }
          b = median;
          continue;
        }
        Ordering::Greater => {
          if a == b {
            // search cannot shrink when the range is already length 0
            // Suggest to the caller that we've found where the closest smaller
            // element is located for possible insertion.
            return Ok(MapGetResult::ClosestLarger(a));
          }
          a = median;
          continue;
        }
        Ordering::Equal => {
          // Found it!
          return Ok(MapGetResult::FoundAt(median));
        }
      }
    }
  }
}
