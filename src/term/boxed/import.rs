use term::boxed::BoxHeader;
use emulator::mfa::MFArity;

pub struct Import {
  header: BoxHeader,
  pub mfarity: MFArity,
  pub is_bif: bool,
}

impl Import {

}
