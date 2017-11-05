//use defs::Word;
use emulator::process::Process;
use term::lterm::LTerm;


pub fn ubif_sminus_2_2(cur_proc: &mut Process,
                       args: &[LTerm]) -> LTerm {
  cur_proc.pid
}
