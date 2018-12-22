use crate::emulator::{code::pointer::VersionedCodePtr, mfa::MFArity};

/// A pointer to a code location: used in funs created with a `fun m:f/a`
/// expression, in module export table and module local functions table.
#[derive(Debug, Clone)]
pub struct Export {
  /// Where the export points to.
  pub mfa: MFArity,
  pub dst: Option<VersionedCodePtr>,
}

impl Export {
  pub fn new(mfa: MFArity) -> Export {
    Export { mfa, dst: None }
  }

  //  pub fn new_code_offset(fa: &FunArity,
  //                         mod_id: &VersionedModuleId,
  //                         offset: usize) -> Export
  //  {
  //    let far_offset = FarCodePointer::new(mod_id, offset);
  //    Export::new(MFArity::new_from_funarity(mod_id.module(), &fa))
  //    //  dst: CallableLocation::Code(far_offset))
  //  }
}
