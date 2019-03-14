use crate::term::{boxed::boxtype::BoxType, classify::TermClass};

// pub type InplaceMapFn = FnMut(*mut boxed::BoxHeader, Term) -> Term;

pub trait TBoxed {
  fn get_class(&self) -> TermClass;
  fn get_type(&self) -> BoxType;

  //  /// For all terms contained in this boxed, run a function and update the data.
  //  fn inplace_map(&mut self, mapfn: &InplaceMapFn);
}
