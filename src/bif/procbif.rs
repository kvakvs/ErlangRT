use emulator::process::Process;
use term::lterm::LTerm;

pub fn ubif_self_0(_cur_proc: &mut Process, _args: &[LTerm]) -> LTerm {
  LTerm::non_value()
}
