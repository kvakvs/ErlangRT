use crate::emulator::{process_flags::ProcessFlags, scheduler::Prio};

#[allow(dead_code)]
pub enum MessageQueueLocation {
  OnHeap,
  OffHeap,
}

pub struct SpawnOptions {
  pub msg_queue: MessageQueueLocation,
  pub prio: Prio,
  // TODO: Use bit flags?
  pub process_flags: ProcessFlags,
}

impl SpawnOptions {
  pub fn default() -> Self {
    Self {
      msg_queue: MessageQueueLocation::OnHeap,
      prio: Prio::Normal,
      process_flags: ProcessFlags::default(),
    }
  }
}
