pub mod dump;
pub mod heap_impl;
pub mod iter;

use rt_defs::Word;


/// Default heap size for constants (literals) when loading a module.
pub const DEFAULT_LIT_HEAP: Word = 8192;
/// Default heap size when spawning a process. (default: 300)
pub const DEFAULT_PROC_HEAP: Word = 8192;


pub use emulator::heap::heap_impl::{Heap, allocate_cons, allocate_tuple, heap_iter};
