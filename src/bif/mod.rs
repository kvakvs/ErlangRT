pub mod gen_bif; // generated
pub mod procbif;

use term::lterm::LTerm;
use emulator::process::Process;
use emulator::mfa::MFArity;

pub type BifFn = fn(cur_proc: &mut Process, args: &[LTerm]) -> LTerm;

pub use bif::procbif::*;


pub fn is_bif(mfa: &MFArity) {

}