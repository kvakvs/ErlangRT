use erlangrt::{command_line_args::ErlStartArgs, lib_main::start_emulator};
use std::env;

fn main() {
  let in_args: Vec<String> = env::args().collect();
  let mut args = ErlStartArgs::new(&in_args);
  args.populate_with(in_args.iter());
  println!("{args:?}");

  // TODO: For windows, support ERL_CONSOLE_MODE, with ERL_EMULATOR_DLL from erlexec.c
  // TODO: For non-Windows, support CERL_DETACHED_PROG?

  // TODO: add -pa, -pz options
  args.search_path = vec![
    "priv/".to_string(),
    // "/home/kv/r20/lib/erts-9.1/ebin/".to_string(),
  ];

  // Get going now
  start_emulator(&mut args);
  println!("erlexec: Finished.");
}
