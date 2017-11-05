use beam::gen_op;
use beam::vm_dispatch::dispatch_op_inline;
use defs::{DispatchResult};
use emulator::code::{opcode, CodePtr};
use emulator::disasm;
use emulator::runtime_ctx::Context;
use emulator::vm::VM;
use emulator::scheduler::SliceResult;


impl VM {
  /// Take a process from scheduler.
  /// Fetch an opcode and execute it.
  /// Reduce the reduction (instruction) count and once it reaches zero, return.
  /// Call dispatch again to schedule another process.
  pub fn dispatch(&mut self) -> bool {
    let mut ctx = Context::new(CodePtr::null());

    let curr_p = match self.scheduler.next_process() {
      None => return false,
      Some(p) => self.scheduler.lookup_pid_mut(&p).unwrap()
    };
    ctx.copy_from(&curr_p.context); // swapin

    loop {
      if cfg!(debug_assertions) { unsafe {
        print!("[exec] ");
        disasm::disasm_op(ctx.ip.get_ptr());
      }}

      // Take next opcode
      let op = opcode::from_memory_word(ctx.fetch());
      assert!(op <= gen_op::OPCODE_MAX,
              "Opcode too big (wrong memory address?) got 0x{:x}", op);

      // Handle next opcode
      match dispatch_op_inline(op, &mut ctx, curr_p) {
        DispatchResult::Yield => {
          curr_p.context.copy_from(&ctx); // swapout
          curr_p.timeslice_result = SliceResult::Yield;
          return true
        },
        DispatchResult::Error => {
          curr_p.context.copy_from(&ctx); // swapout
//          let pid = curr_p.pid;
//          let exc_type = curr_p.error_type;
//          let exit_reason = curr_p.error_reason;
//          self.scheduler.exit_process(pid, exc_type, exit_reason);
          curr_p.timeslice_result = SliceResult::Exception;
          return false
        },
        DispatchResult::Normal => {
          curr_p.timeslice_result = SliceResult::None;
        }, // keep looping
      }
    } // end loop
  }
}
