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

      // Display a warning if boxed points outside the current heap
      match primary::get_tag(v) {
        x if x == primary::Tag::Box
            || x == primary::Tag::Header
            || x == primary::Tag::Cons => {
          let p = primary::pointer(v);
          if p < self.begin() || p >= self.end() {
            print!("[bounds!] ")
          }
        }
        _ => {}
      }

      if cfg!(target_pointer_width = "32") {
        println!("{:08p}: val=0x{:08x} {}", addr, v, LTerm::from_raw(v))
      } else if cfg!(target_pointer_width = "64") {
        println!("{:08p}: val=0x{:016x} {}", addr, v, LTerm::from_raw(v))
      } else {
        panic!("Pointer width is expected to be 32 or 64")
      }
    }
  }

}