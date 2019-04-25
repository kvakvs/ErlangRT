//! Debug tool to display Erlang heap contents.
use crate::{defs::Word, emulator::heap::Heap, term::value::PrimaryTag};

impl Heap {
  /// Print heap contents
  // This function is not used sometimes, this is fine to ignore dead code
  #[allow(dead_code)]
  pub unsafe fn dump(&self) {
    let mut data_p = self.heap_iter();
    while let Some(addr) = data_p.next() {
      let val_at_addr = *addr;
      let mut output = String::new();

      if cfg!(target_pointer_width = "32") {
        output += &format!(
          "{:08p}: val=0x{:08x} {}",
          addr,
          val_at_addr.raw(),
          val_at_addr
        )
      } else if cfg!(target_pointer_width = "64") {
        output += &format!(
          "{:08p}: val=0x{:016x} {}",
          addr,
          val_at_addr.raw(),
          val_at_addr
        )
      } else {
        panic!("Pointer width is expected to be 32 or 64")
      }

      // Display a warning if boxed points outside the current heap
      if let PrimaryTag::BOX_PTR = val_at_addr.get_term_tag() {
        let p = val_at_addr.get_box_ptr() as *const Word;
        if p < self.get_heap_start_ptr() || p >= self.get_heap_top_ptr() {
          output += " <- heap bounds!";
        }
      }

      println!("{}", output)
    } // for
  }
}
