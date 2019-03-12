use crate::term::{boxed::boxtype::BoxType, classify::TermClass};

pub trait TBoxed {
  fn get_class(&self) -> TermClass;
  fn get_type(&self) -> BoxType;
}
