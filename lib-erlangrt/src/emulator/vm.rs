//! Implements virtual machine, as a collection of processes and their
//! registrations, schedulers, ETS tables and atom table etc.

use crate::{
  command_line_args::ErlStartArgs,
  defs::Word,
  emulator::{
    code_srv::CodeServer,
    mfa::ModFunArgs,
    process::Process,
    process_registry::ProcessRegistry,
    scheduler::{Scheduler},
    spawn_options::SpawnOptions,
  },
  fail::RtResult,
  term::value::*,
};
use crate::emulator::process_flags;
use crate::emulator::heap::Heap;

/// VM environment, heaps, tables, processes all goes here.
/// Atoms are a global API in `atom.rs`.
/// Code server is a global API in `code_srv.rs`.
pub struct VM {
  /// Pid counter increments every time a new process is spawned
  pid_counter: Word,

  /// Contains all loaded modules and manages versions
  pub code_server: CodeServer,

  pub scheduler: Scheduler,
  pub processes: ProcessRegistry,
  pub binary_heap: Heap,
}

const BINARY_HEAP_CAPACITY: usize = 65536; // 64k*8 = 512kb

impl VM {
  /// Create a VM, multiple VMs can be created but atom table and code server
  /// will be shared (global).
  pub fn new(args: &mut ErlStartArgs) -> VM {
    VM {
      code_server: CodeServer::new(args),
      pid_counter: 0,
      scheduler: Scheduler::new(),
      processes: ProcessRegistry::new(),
      binary_heap: Heap::new(BINARY_HEAP_CAPACITY),
    }
  }

  /// Dirty trick to not have to dynamically borrow scheduler via
  /// `RefCell<Box<>>` because schedulers live just as long as the VM itself.
  #[allow(dead_code)]
  pub fn get_scheduler_p(&self) -> *mut Scheduler {
    let p = &self.scheduler as *const Scheduler;
    p as *mut Scheduler
  }

  /// Dirty trick to not have to dynamically borrow code server via
  /// `RefCell<Box<>>` because code server lives just as long as the VM itself.
  pub fn get_code_server_p(&self) -> *mut CodeServer {
    let p = &self.code_server as *const CodeServer;
    p as *mut CodeServer
  }

  /// Spawn a new process, create a new pid, register the process and jump to
  /// the MFA specified. Arguments are copies into the new process heap and
  /// stored into the registers.
  pub fn create_process(
    &mut self,
    parent: Term,
    mfargs: &ModFunArgs,
    spawn_opts: &SpawnOptions,
  ) -> RtResult<Term> {
    let pid_c = self.pid_counter;
    self.pid_counter += 1;

    let pid = Term::make_local_pid(pid_c);
    let mfarity = mfargs.get_mfarity()?;
    let cs = self.get_code_server_p();
    let mut p0 = Process::new(pid, parent, &mfarity, spawn_opts, unsafe { &mut (*cs) })?;

    // Error may happen here due to arg term copy error
    p0.set_spawn_args(&mfargs)?;

    self.register_new_process(pid, p0);
    Ok(pid)
  }

  pub fn spawn_system_process(
    &mut self,
    parent: Term,
    mfargs: &ModFunArgs,
    mut spawn_opts: SpawnOptions,
  ) -> RtResult<Term> {
    let mfarity = mfargs.get_mfarity()?;
    // Fail if MFA not found, otherwise continue
    let _ = self.code_server.lookup_mfa(&mfarity, false)?;
    spawn_opts.process_flags.set(process_flags::SYSTEM_PROCESS);
    self.create_process(parent, mfargs, &spawn_opts)
  }

  pub fn register_new_process(&mut self, pid: Term, mut proc: Process) {
    proc.owned_by_scheduler = (&mut self.scheduler) as *mut Scheduler;
    self.processes.insert(pid, proc);
    self.scheduler.enqueue(&mut self.processes, pid);
  }

  /// Run the VM loop (one time slice), call this repeatedly to run forever.
  /// Time slice ends when a current process yields or when reduction count
  /// reaches zero.
  #[inline]
  pub fn tick(&mut self) -> RtResult<bool> {
    self.dispatch()
  }
}
