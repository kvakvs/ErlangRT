use bif::BifResult;
use emulator::process::Process;
use term::lterm::LTerm;


pub fn ubif_self_0(cur_proc: &mut Process, _args: &[LTerm]) -> BifResult
{
  BifResult::Value(cur_proc.pid)
}
