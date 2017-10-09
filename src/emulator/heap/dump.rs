//! Debug tool to display Erlang heap contents.
use defs::Word;
use emulator::heap::{Heap, DataPtr};
use term::lterm::LTerm;
use term::raw::{RawConsMut, RawTupleMut, RawBignum};
use term::primary;


impl Heap {
  /// Print heap contents
  pub unsafe fn dump(&self) {
    for data_p in self.iter() {
      let DataPtr::Ptr(addr) = data_p;
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
        x if x == primary::TAG_BOX
            || x == primary::TAG_HEADER
            || x == primary::TAG_CONS => {
          let p = primary::pointer(v);
          if p < self.begin() || p >= self.end() {
            output += " <- heap bounds!";
          }
        }
        _ => {}
      }

      println!("{}", output)
    } // for
  }

}