pub enum MTermType {
  Integer,
  Atom,
  Tuple,
  List,
}

/// Term in memory
pub trait MTerm {
  /// From header word return the contents type
  fn get_type() -> MTermType;
}
