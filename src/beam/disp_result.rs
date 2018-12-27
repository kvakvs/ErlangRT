/// Enum is used by VM dispatch handlers for opcodes to indicate whether to
/// continue, yield (take next process in the queue) or interrupt process
/// on error (to return error use Hopefully's Error::Exception/2)
#[allow(dead_code)]
pub enum DispatchResult {
  // Continue running
  Normal,
  // Process falls asleep (loses its running status and waits for its turn)
  Yield,
  // Process is done
  Finished,
}
