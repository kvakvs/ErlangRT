use crate::{
  beam::{disp_result::DispatchResult, gen_op, vm_dispatch::dispatch_op_inline},
  emulator::{code::opcode, disasm, scheduler::SliceResult, vm::VM},
  fail::{Error, RtResult},
};

// fn module() -> &'static str { "vm_loop: " }

impl VM {
  /// Take a process from scheduler.
  /// Fetch an opcode and execute it.
  /// Reduce the reduction (instruction) count and once it reaches zero, return.
  /// Call dispatch again to schedule another process.
  pub fn dispatch(&mut self) -> RtResult<bool> {
    let scheduler = self.get_scheduler_p();
    let curr_p = match unsafe { (*scheduler).next_process() } {
      None => return Ok(false),
      Some(p) => unsafe { (*scheduler).lookup_pid_mut(p).unwrap() },
    };
    println!("+ Scheduler: switching to {}", curr_p.pid);

    // Ugly borrowing the context from the process, but we guarantee that the
    // borrow will not outlive the owning process or we pay the harsh price
    // debugging SIGSEGVs.
    let ctx_p = curr_p.get_context_p();
    let mut ctx = unsafe { &mut (*ctx_p) };

    let cs = self.get_code_server_p();
    loop {
      if cfg!(debug_assertions) {
        print!(" ↳ ");
        unsafe {
          disasm::disasm_op(ctx.ip.get(), &(*cs));
        }
      }

      // Take next opcode
      let op = opcode::from_memory_word(ctx.fetch());
      assert!(
        op <= gen_op::OPCODE_MAX,
        "Opcode too big (wrong memory address?) got 0x{:x}",
        op.get()
      );

      // Handle next opcode
      let disp_result = dispatch_op_inline(self, op, &mut ctx, curr_p);
      if let Err(Error::Exception(exc_type, exc_reason)) = disp_result {
        println!("vm: Exception type={:?} reason={}", exc_type, exc_reason);
        curr_p.exception(exc_type, exc_reason);
        curr_p.timeslice_result = SliceResult::Exception;
        return Ok(true);
      }

      match disp_result? {
        DispatchResult::Yield => {
          curr_p.timeslice_result = SliceResult::Yield;
          return Ok(true);
        }
        DispatchResult::Normal => {
          curr_p.timeslice_result = SliceResult::None;
        } // keep looping
      }
    } // end loop
  }
}
