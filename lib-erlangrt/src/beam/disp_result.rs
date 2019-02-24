// TODO: Is this accidentally same thing as SliceResult?
pub enum YieldType {
  /// The process gives up running for the moment, and is placed at the end
  /// of the run queue.
  EndOfTheQueue,
  /// The process gives up running for infinite receive or a similar reason.
  InfiniteWait,
}

/// Enum is used by VM dispatch handlers for opcodes to indicate whether to
/// continue, yield (take next process in the queue) or interrupt process
/// on error (to return error use Hopefully's RtErr::Exception/2)
#[allow(dead_code)]
pub enum DispatchResult {
  // Continue running, advance IP to the next opcode
  Normal,
  // Process falls asleep (loses its running status and waits for its turn)
  Yield(YieldType),
  // Process is done
  Finished,
}
