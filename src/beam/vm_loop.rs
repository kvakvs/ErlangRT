use emulator::vm::VM;

impl VM {
  /// Take a process from scheduler.
  /// Fetch an opcode and execute it.
  /// Reduce the reduction (instruction) count and once it reaches zero, return.
  /// Call dispatch again to schedule another process.
  pub fn dispatch(&mut self) -> bool {
    let current_p = self.scheduler.next();
    true
  }
}
