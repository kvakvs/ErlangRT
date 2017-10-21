use defs::Word;

pub enum HeapObjType {
  /// Maps to `emulator::heap::ho_import::HOImport`
  Import,
}

/// Used to identify heap object type on heap
pub struct HeapObjClass {
  pub obj_type: HeapObjType,
  pub dtor: fn(this: *mut Word) -> (),
}
