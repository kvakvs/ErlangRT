//! Debug tool to display Erlang heap contents.
use emulator::heap::{Heap, heap_iter};
use rt_defs::heap::iter::IHeapIterator;
use rt_defs::heap::ptr::DataPtr;
use rt_defs::heap::{IHeap};
use term::lterm::*;
use term::primary;


impl Heap {

  /// Print heap contents
  #[allow(dead_code)]
  // This function is not used sometimes, this is fine
  pub unsafe fn dump(&self) {
    let mut data_p = heap_iter(self);
    while let Some(DataPtr(addr)) = data_p.next() {
      let val_at_addr = *addr;
      let mut output = String::new();

      if cfg!(target_pointer_width = "32") {
        output += &format!("{:08p}: val=0x{:08x} {}",
                          addr, val_at_addr, LTerm::from_raw(val_at_addr))
      } else if cfg!(target_pointer_width = "64") {
        output += &format!("{:08p}: val=0x{:016x} {}",
                          addr, val_at_addr, LTerm::from_raw(val_at_addr))
      } else {
        panic!("Pointer width is expected to be 32 or 64")
      }

      // Display a warning if boxed points outside the current heap
      match primary::get_tag(val_at_addr) {
        x if x == primary::TAG_BOX || x == primary::TAG_CONS => {
          let p = primary::pointer(val_at_addr);
          if p < self.heap_begin() || p >= self.heap_end() {
            output += " <- heap bounds!";
          }
        }
        _ => {}
      }

      println!("{}", output)
    } // for
  }

}