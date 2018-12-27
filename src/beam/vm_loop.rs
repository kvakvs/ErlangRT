use crate::{
  beam::{disp_result::DispatchResult, gen_op, vm_dispatch::dispatch_op_inline},
  emulator::{disasm, scheduler::SliceResult, vm::VM},
  fail::{Error, RtResult},
};

// fn module() -> &'static str { "vm_loop: " }

impl VM {
  /// Take a process from scheduler.
  /// Fetch an opcode and execute it.
  /// Reduce the reduction (instruction) count and once it reaches zero, return.
  /// Call dispatch again to schedule another process.
  ///
  /// Returns: `false` if VM found no process to run, `true` if the process has
  /// used its time slice and wants to run another.
  pub fn dispatch(&mut self) -> RtResult<bool> {
    let scheduler = self.get_scheduler_p();
    let curr_p = match unsafe { (*scheduler).next_process() } {
      None => return Ok(false),
      Some(p) => unsafe { (*scheduler).lookup_pid_mut(p).unwrap() },
    };

    // Ugly borrowing the context from the process, but we guarantee that the
    // borrow will not outlive the owning process or we pay the harsh price
    // debugging SIGSEGVs.
    let ctx_p = curr_p.get_context_p();
    let mut ctx = unsafe { &mut (*ctx_p) };
    ctx.swap_in(); // tell the context, that it is active now

    let cs = self.get_code_server_p();

    // Fetch some opcodes, Execute some opcodes
    //
    loop {
      if cfg!(feature = "trace_opcode_execution") {
        print!(" ↳ ");
        unsafe {
          disasm::disasm_op(ctx.ip.get(), &(*cs));
        }
      }

      // Take next opcode
      let op = ctx.fetch_opcode();
      debug_assert!(
        op <= gen_op::OPCODE_MAX,
        "Opcode too big (wrong memory address?) got 0x{:x}",
        op.get()
      );

      // Handle next opcode
      let disp_result = match dispatch_op_inline(self, op, &mut ctx, curr_p) {
        Err(Error::Exception(exc_type, exc_reason)) => {
          println!("vm: Exception type={} reason={}", exc_type, exc_reason);
          curr_p.set_exception(exc_type, exc_reason);
          curr_p.timeslice_result = SliceResult::Exception;
          return Ok(true);
        }
        other => other?,
      };

      match disp_result {
        DispatchResult::Yield => {
          curr_p.timeslice_result = SliceResult::Yield;
          return Ok(true);
        }
        DispatchResult::Normal => {
          curr_p.timeslice_result = SliceResult::None;
        } // keep looping
        DispatchResult::Finished => {
          curr_p.timeslice_result = SliceResult::Finished;
        }
      }

      if ctx.reductions <= 0 {
        // Out of reductions, just give up and let another one run
        curr_p.timeslice_result = SliceResult::None;
        return Ok(true);
      }
    } // end loop
  }
}
