use crate::term::classify::TermClass;
use crate::term::boxed::boxtype::BoxType;

pub trait TBoxed {
  fn get_class(&self) -> TermClass;
  fn get_type(&self) -> BoxType;
}
