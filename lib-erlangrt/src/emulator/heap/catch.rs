use crate::defs::Word;

pub struct NextCatchResult {
  /// Catch jump pointer, where exception handling code (Erlang `catch` /
  /// Asm `try_case`) is located
  pub loc: *const Word,
  /// How many stack cells have to be dropped
  pub stack_drop: usize,
}
