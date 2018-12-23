use crate::{
  emulator::{atom, mfa::MFASomething, scheduler::Prio, vm::VM},
  term::lterm::*,
};

use crate::emulator::mfa::Args;
use std::{thread, time};

/// Entry point for the command-line interface
#[inline]
pub fn entrypoint() {
  if cfg!(feature = "r19") {
    println!("Erlang Runtime (compat OTP 19)");
    panic!("Support for R19 is unfinished and probably never will be finished.")
  }
  if cfg!(feature = "r20") {
    println!("Erlang Runtime (compat OTP 20)");
  }

  let mut beam_vm = VM::new();

  let mfargs = MFASomething::new(
    atom::from_str("test2"),
    atom::from_str("test"),
    Args::AsList(LTerm::nil()),
  );
  let _rootp = beam_vm
    .create_process(LTerm::nil(), &mfargs, Prio::Normal)
    .unwrap();

  println!("Process created. Entering main loop...");
  while beam_vm.tick().unwrap() {
    thread::sleep(time::Duration::from_millis(0));
  }
}
