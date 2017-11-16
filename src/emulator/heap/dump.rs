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
    loop {
      let DataPtr(addr) = match data_p.next() {
        Some(p0) => p0,
        None => break,
      };
      let v = *addr;
      let mut output = String::new();

      if cfg!(target_pointer_width = "32") {
        output += &format!("{:08p}: val=0x{:08x} {}",
                          addr, v, LTerm::from_raw(v))
      } else if cfg!(target_pointer_width = "64") {
        output += &format!("{:08p}: val=0x{:016x} {}",
                          addr, v, LTerm::from_raw(v))
      } else {
        panic!("Pointer width is expected to be 32 or 64")
      }

      // Display a warning if boxed points outside the current heap
      match primary::get_tag(v) {
        x if x == primary::TAG_BOX || x == primary::TAG_CONS => {
          let p = primary::pointer(v);
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