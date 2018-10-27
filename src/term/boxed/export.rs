use term::boxed::BoxHeader;
use emulator::export;

pub struct Export {
  header: BoxHeader,
  pub exp: export::Export
}

impl Export {}
