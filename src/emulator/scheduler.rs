//! Code related to task scheduling and priorities.
use crate::{
  emulator::{
    gen_atoms,
    process::{Process, ProcessError},
  },
  defs::{ExceptionType, Word},
  term::lterm::*,
};
use std::collections::{HashMap, VecDeque};


fn module() -> &'static str {
  "scheduler: "
}


#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
pub enum Prio {
  /// Runs when no more jobs to take or at 8x disadvantage to normal
  Low = 0,
  /// Most of user processes run at this priority
  Normal = 1,
  /// Takes priority always over everything else
  High = 2,
}


/// Enum identifies current registration of the process
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[allow(dead_code)]
pub enum Queue {
  None,
  //PendingTimers,
  High,
  Normal,
  Low,
  TimedWait,
  InfiniteWait,
}


#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[allow(dead_code)]
pub enum SliceResult {
  None,
  /// Process willingly gave up run queue (ended the timeslice without events)
  Yield,
  /// Process entered infinite or timed wait during the last timeslice
  Wait,
  /// Process normally finished during the last timeslice
  Finished,
  /// Error, exit or throw occured during the last timeslice
  Exception,
}


/// How many Normal processes can be scheduled before Low gets to run.
const NORMAL_ADVANTAGE: Word = 8;


/// Maintains run queues for different priorities and allows queuing processes,
/// suspending processes, work balancing (TODO), etc.
pub struct Scheduler {
  // This is the naive implementation of run queues.
  // A better approach would be to build an intrusive double linked list through
  // every process in the queue (as done by the original ERTS).
  queue_low: VecDeque<LTerm>,
  queue_normal: VecDeque<LTerm>,
  queue_high: VecDeque<LTerm>,

  /// A counter used to skip some schedulings for low processes
  advantage_count: Word,

  /// Currently selected process
  current: Option<LTerm>,

  //  /// Wait set for infinitely suspended processes (in endless receive)
  //  wait_inf: HashSet<LTerm>,
  //  /// Wait set for timed suspended processes (waiting for a timer)
  //  wait_timed: HashSet<LTerm>,
  /// Dict of pids to process boxes. Owned by the scheduler
  processes: HashMap<LTerm, Process>,
}


impl Scheduler {
  pub fn new() -> Scheduler {
    Scheduler {
      queue_low: VecDeque::new(),
      queue_normal: VecDeque::new(),
      queue_high: VecDeque::new(),

      advantage_count: 0,
      current: None,

      //      wait_inf: HashSet::new(),
      //      wait_timed: HashSet::new(),
      processes: HashMap::new(),
    }
  }


  /// Register a process `proc_` in the process table and also queue it for
  /// execution. This is invoked by vm when a new process is spawned.
  pub fn add(&mut self, pid: LTerm, proc_: Process) {
    self.processes.insert(pid, proc_);
    self.queue(pid);
  }


  /// Queue a process by its pid. Will `panic!` if the process doesn't exist
  /// or is already queued.
  pub fn queue(&mut self, pid: LTerm) {
    assert!(pid.is_local_pid());

    let prio = {
      // Lookup the pid
      let p = self.lookup_pid(pid).unwrap();
      assert_eq!(p.current_queue, Queue::None);
      p.prio
    };

    match prio {
      Prio::Normal => self.queue_normal.push_back(pid),
      Prio::Low => self.queue_low.push_back(pid),
      Prio::High => self.queue_high.push_back(pid),
    }
  }


  /// Get another process from the run queue for this scheduler.
  pub fn next_process(&mut self) -> Option<LTerm> {
    let current_pid = self.current;
    if let Some(curr_pid) = current_pid {
      let timeslice_result = {
        let curr = self.lookup_pid(curr_pid).unwrap();
        assert_eq!(curr.current_queue, Queue::None);
        curr.timeslice_result
      };

      match timeslice_result {
        SliceResult::Yield | SliceResult::None => {
          self.queue(curr_pid);
          self.current = None
        }

        SliceResult::Finished => {
          let err = ProcessError::Exception(ExceptionType::Exit, gen_atoms::NORMAL);
          self.exit_process(curr_pid, err)
        }

        SliceResult::Exception => {
          let p_error = {
            let curr = self.lookup_pid(curr_pid).unwrap();
            assert!(curr.is_failed());
            curr.error
          };
          self.exit_process(curr_pid, p_error);
          self.current = None
        }

        SliceResult::Wait => {}
      }
    } // if self.current

    // Now try and find another process to run
    while self.current.is_none() {
      // TODO: monotonic clock
      // TODO: wait lists
      // TODO: network checks

      // See if any are waiting in realtime (high) priority queue
      let mut next_pid: Option<LTerm> = None;
      if !self.queue_high.is_empty() {
        next_pid = self.queue_high.pop_front()
      } else if self.advantage_count < NORMAL_ADVANTAGE {
        if !self.queue_normal.is_empty() {
          next_pid = self.queue_normal.pop_front()
        } else if !self.queue_low.is_empty() {
          next_pid = self.queue_low.pop_front()
        }
        self.advantage_count += 1;
      } else {
        if !self.queue_low.is_empty() {
          next_pid = self.queue_low.pop_front()
        } else if !self.queue_normal.is_empty() {
          next_pid = self.queue_normal.pop_front()
        }
        self.advantage_count = 0;
      };

      if next_pid.is_some() {
        let next_pid1 = next_pid.unwrap();
        //        {
        //          let next_p = self.lookup_pid(&next_pid1).unwrap();
        //          println!("{} next() queue {}", module(), next_p.pid);
        //        }

        self.current = Some(next_pid1)
      }
    }

    self.current
  }


  /// Get a read-only process, if it exists. Return `None` if we are sorry.
  pub fn lookup_pid(&self, pid: LTerm) -> Option<&Process> {
    assert!(pid.is_local_pid());
    self.processes.get(&pid)
  }


  /// Get a reference to process, if it exists. Return `None` if we are sorry.
  pub fn lookup_pid_mut(&mut self, pid: LTerm) -> Option<&mut Process> {
    assert!(pid.is_local_pid());
    self.processes.get_mut(&pid)
  }


  pub fn exit_process(&mut self, pid: LTerm, e: ProcessError) {
    // assert that process is not in any queue
    {
      let p = self.lookup_pid_mut(pid).unwrap();
      assert_eq!(p.current_queue, Queue::None);
    }

    // root process exits with halt()
    // assert!(p.get_registered_name() != atom::INIT);

    // TODO: ets tables
    // TODO: notify monitors
    // TODO: cancel known timers who target this process
    // TODO: notify links
    // TODO: unregister name if registered
    // TODO: if pending timers - become zombie and sit in pending timers queue
    println!(
      "{}exit_process {} e={}, result x0=?",
      module(),
      pid,
      e //, p.runtime_ctx.regs[0]
    );

    //  m_inf_wait.erase(p);
    //  m_timed_wait.erase(p);
    assert!(!self.queue_normal.contains(&pid));
    assert!(!self.queue_low.contains(&pid));
    assert!(!self.queue_high.contains(&pid));
    self.processes.remove(&pid);
  }
}
