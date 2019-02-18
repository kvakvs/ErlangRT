#[derive(Debug, Clone, Copy)]
pub struct ProcessFlag(usize);

pub const TRAP_EXIT: ProcessFlag = ProcessFlag(1usize << 0);
pub const SYSTEM_PROCESS: ProcessFlag = ProcessFlag(1usize << 1);

#[derive(Debug, Clone, Copy)]
pub struct ProcessFlags(usize);

impl ProcessFlags {
  pub fn default() -> Self {
    ProcessFlags(0)
  }

  #[inline]
  pub fn get(&mut self, flag: ProcessFlag) -> bool {
    self.0 & flag.0 != 0
  }

  pub fn read_and_set(&mut self, flag: ProcessFlag, value: bool) -> bool {
    let old_val = self.get(flag);
    if value {
      self.0 |= flag.0;
    } else {
      self.0 &= !flag.0;
    }
    old_val
  }

  #[allow(dead_code)]
  pub fn set_value(&mut self, flag: ProcessFlag, value: bool) {
    if value {
      self.0 |= flag.0;
    } else {
      self.0 &= !flag.0;
    }
  }

  #[inline]
  pub fn set(&mut self, flag: ProcessFlag) {
    self.0 |= flag.0;
  }

  #[allow(dead_code)]
  #[inline]
  pub fn clear(&mut self, flag: ProcessFlag) {
    self.0 &= !flag.0;
  }
}
