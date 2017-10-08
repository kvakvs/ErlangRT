use beam::gen_op;
use beam::vm_dispatch::dispatch_op_inline;
use defs::{Word, DispatchResult};
use emulator::code::{CodePtr};
use emulator::vm::VM;

use std::mem::transmute;

impl VM {
  /// Take a process from scheduler.
  /// Fetch an opcode and execute it.
  /// Reduce the reduction (instruction) count and once it reaches zero, return.
  /// Call dispatch again to schedule another process.
  pub fn dispatch(&mut self) -> bool {
    loop {
      let curr_p = match self.scheduler.next() {
        None => return false,
        Some(p) => self.scheduler.lookup_pid_mut(&p).unwrap()
      };
      let mut ctx = &mut curr_p.context;

      // Take next opcode

      let op_w = ctx.fetch();
      assert!(op_w <= gen_op::OPCODE_MAX);
      let op = unsafe { transmute::<usize, gen_op::OPCODE>(op_w) };

      // Handle next opcode
      match dispatch_op_inline(op, &mut ctx) {
        DispatchResult::Yield => return true,
        DispatchResult::Error => return false,
        DispatchResult::Normal => {}, // keep looping
      }
    } // end loop
  }

}
