//use defs::Word;
use emulator::process::Process;
use term::lterm::LTerm;


pub fn ubif_self_0(cur_proc: &mut Process,
                   _args: &[LTerm]) -> LTerm {
  cur_proc.pid
}
