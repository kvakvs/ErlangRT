pub mod dump;
pub mod heap_impl;
pub mod heapobj;
pub mod ho_import;
pub mod iter;

use defs::Word;

/// Default heap size for constants (literals) when loading a module.
pub const DEFAULT_LIT_HEAP: Word = 1024;
/// Default heap size when spawning a process.
pub const DEFAULT_PROC_HEAP: Word = 300;


pub use emulator::heap::heap_impl::*;
