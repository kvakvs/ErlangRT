use emulator::code::FarCodePointer;
use emulator::code_srv::module_id::VersionedModuleId;
use emulator::function::CallableLocation;
use emulator::mfa::MFArity;
use emulator::funarity::FunArity;


/// A pointer to a code location: used in funs created with a `fun m:f/a`
/// expression, in module export table and module local functions table.
#[derive(Debug, Copy, Clone)]
pub struct Export {
  /// Where the export points to.
  pub mfa: MFArity,
  /// Cached value for faster calls.
  pub dst: CallableLocation,
}


impl Export {
  pub fn new(mfa: &MFArity) -> Export {
    Export { mfa: *mfa, dst: CallableLocation::NeedUpdate }
  }


  pub fn new_code_offset(fa: &FunArity,
                         mod_id: &VersionedModuleId,
                         offset: usize) -> Export {
    let far_offset = FarCodePointer::new(mod_id, offset);
    Export {
      mfa: MFArity::new_from_funarity(mod_id.module(), &fa),
      dst: CallableLocation::Code(far_offset)
    }
  }
}
