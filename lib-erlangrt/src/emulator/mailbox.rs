use crate::term::lterm::*;

pub struct ProcessMailbox {
  inbox: Vec<LTerm>,
  // TODO: Some structure on proc heap?
  read_index: usize,
}

impl ProcessMailbox {
  pub fn new() -> Self {
    Self {
      inbox: Vec::with_capacity(32),
      read_index: 0,
    }
  }

  #[inline]
  pub fn have_unread_messages(&self) -> bool {
    self.read_index < self.inbox.len()
  }

//  #[inline]
//  pub fn is_empty(&self) -> bool {
//    self.inbox.is_empty()
//  }

  /// Copy a message and put into process mailbox.
  /// Assumes: the message is already copied to receiving process heap.
  pub fn put(&mut self, message: LTerm) {
    self.inbox.push(message);
  }

  /// Read message at the current receive pointer.
  pub fn get_current(&mut self) -> Option<LTerm> {
    if self.inbox.is_empty() {
      return None;
    }

    let mri = self.read_index;
    debug_assert!(mri < self.inbox.len());

    let val = self.inbox[mri];
    debug_assert!(val.is_value());
    Some(val)
  }

  // TODO: This is ugly, do proper mailbox algorithm impl here
  pub fn step_over(&mut self) {
    // Guard
    if self.inbox.is_empty() {
      self.read_index = 0;
      return;
    }

    let mut mri = self.read_index;
    // remember starting pos to know if we traversed all messages and all of
    // them were nonvalues
    let starting_pos = mri;
    let max_mri = self.inbox.len();
    // Increase mail receive index over nonvalues (received values) until
    // we hit the end of the mailbox
    while self.inbox[mri].is_non_value() {
      mri += 1;
      if mri == max_mri {
        mri = 0;
      }
      if mri == starting_pos {
        // Done a full loop around mailbox and all values were non-values
        self.inbox.clear();
        self.read_index = 0;
        return;
      }
    }
    self.read_index = mri;
  }

  // Remove value from current mailbox position and return it, move pointer
  // forward.
  pub fn remove_current(&mut self) -> LTerm {
    let mri = self.read_index;
    let val = self.inbox[mri];
    self.inbox[mri] = LTerm::non_value();
    self.step_over();
    val
  }
}
