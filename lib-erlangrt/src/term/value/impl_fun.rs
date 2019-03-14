use crate::term::{boxed, value::Term};

impl Term {
  // === === EXPORT === ===
  //

  /// Check whether a value is a boxed export (M:F/Arity triple).
  pub fn is_export(self) -> bool {
    self.is_boxed_of_type(boxed::BOXTYPETAG_EXPORT)
  }

  // === === FUN / CLOSURE === ===
  //

  /// Check whether a value is a boxed fun (a closure or export).
  pub fn is_fun(self) -> bool {
    self.is_boxed_of_(|t| t == boxed::BOXTYPETAG_CLOSURE || t == boxed::BOXTYPETAG_EXPORT)
  }

  /// Check whether a value is a boxed fun (a closure or export).
  pub fn is_fun_of_arity(self, a: usize) -> bool {
    if !self.is_boxed() {
      return false;
    }

    let box_ptr = self.get_box_ptr::<boxed::BoxHeader>();
    let trait_ptr = unsafe { (*box_ptr).get_trait_ptr() };
    let box_type = unsafe { (*trait_ptr).get_type() };

    match box_type {
      boxed::BOXTYPETAG_CLOSURE => {
        let closure_p = box_ptr as *const boxed::Closure;
        unsafe { (*closure_p).mfa.arity - (*closure_p).nfrozen == a }
      }
      boxed::BOXTYPETAG_EXPORT => {
        let expt_p = box_ptr as *const boxed::Export;
        unsafe { (*expt_p).exp.mfa.arity == a }
      }
      _ => false,
    }
  }
}
