pub mod procbif;

use term::lterm::LTerm;
use emulator::process::Process;

pub type BifFn = fn(cur_proc: &mut Process, args: &[LTerm]) -> LTerm;

pub use bif::procbif::*;
