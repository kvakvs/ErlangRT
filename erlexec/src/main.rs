use std::env;
use crate::erlargs::ErlArgs;

mod erlargs;

fn main() {
  // let in_args: Vec<String> = env::args().collect();
  let mut args = ErlArgs::new();
  args.populate_with(env::args());
  println!("{:?}", args);

  // TODO: For windows, support ERL_CONSOLE_MODE, with ERL_EMULATOR_DLL from erlexec.c
  // TODO: For non-Windows, support CERL_DETACHED_PROG?
}
