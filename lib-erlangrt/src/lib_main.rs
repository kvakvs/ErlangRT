use crate::{
  command_line_args::ErlStartArgs,
  emulator::{
    atom,
    mfa::{Args, MFASomething},
    scheduler::Prio,
    vm::VM,
  },
  term::lterm::*,
};
use std::{
  io::{stdout, Write},
  thread, time,
};

/// Entry point for the command-line interface. Pre-parse command line args
/// by calling StartArgs methods, or just use default constructed StartArgs.
pub fn start_emulator(args: &mut ErlStartArgs) {
  if cfg!(feature = "r20") {
    println!("Erlang Runtime (compat OTP 20)");
  }
  if cfg!(feature = "r21") {
    println!("Erlang Runtime (compat OTP 21)");
  }

  let mut beam_vm = VM::new(args);

  let mfargs = MFASomething::new(
    atom::from_str("init"),
    atom::from_str("boot"),
    Args::AsList(LTerm::nil()),
  );
  let _rootp = beam_vm
    .create_process(LTerm::nil(), &mfargs, Prio::Normal)
    .unwrap();

  println!("Process created. Entering main loop...");
  while beam_vm.tick().unwrap() {
    thread::sleep(time::Duration::from_millis(0));
  }
  stdout().flush().unwrap();
}
