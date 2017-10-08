use beam::vm_dispatch::dispatch_op_inline;
use defs::Word;
use emulator::code::{CodePtr};
use emulator::vm::VM;

impl VM {
  /// Take a process from scheduler.
  /// Fetch an opcode and execute it.
  /// Reduce the reduction (instruction) count and once it reaches zero, return.
  /// Call dispatch again to schedule another process.
  pub fn dispatch(&mut self) -> bool {
    let curr_p = match self.scheduler.next() {
      None => return false,
      Some(p) => self.scheduler.lookup_pid_mut(&p).unwrap()
    };
    let mut ctx = &mut curr_p.context;
    let op = ctx.fetch();
    dispatch_op_inline(op, &mut ctx);
    true
  }
}
