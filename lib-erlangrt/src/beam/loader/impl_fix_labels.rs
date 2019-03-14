use crate::{
  beam::loader::{LoaderState, PatchLocation},
  fail::RtResult,
  term::{
    boxed,
    value::{self, Term},
  },
};

impl LoaderState {
  /// Analyze the code and replace label values with known label locations.
  pub fn fix_labels(&mut self) -> RtResult<()> {
    // Postprocess self.replace_labels, assuming that at this point labels exist
    let mut repl = Vec::<PatchLocation>::new();
    core::mem::swap(&mut repl, &mut self.replace_labels);

    for ploc in &repl {
      match *ploc {
        PatchLocation::PatchCodeOffset(cmd_offset) => {
          let val = Term::from_raw(self.code[cmd_offset]);
          self.code[cmd_offset] = self.resolve_loadtime_label(val).raw()
        }

        PatchLocation::PatchJumpTable(jtab) => {
          let jtab = jtab.get_box_ptr_mut::<boxed::JumpTable>();
          unsafe {
            let count = (*jtab).get_count();
            for i in 0..count {
              // update every location
              let val = (*jtab).get_location(i);
              (*jtab).set_location(i, self.resolve_loadtime_label(val))
            }
          }
        }
      } // match
    } // for ploc
    Ok(())
  }


  /// Helper for `postprocess_fix_label`, takes a word from code memory or from
  /// a jump table, resolves as if it was a label index, and returns a value
  /// to be put back into memory.
  fn resolve_loadtime_label(&self, val: Term) -> Term {
    assert!(val.is_loadtime() && val.get_loadtime_tag() == value::SPECIAL_LT_LABEL);

    // Convert from Term smallint to integer and then to labelid
    let unfixed = val.get_loadtime_val();

    // Zero label id means no location, so we will store NIL [] there
    if unfixed > 0 {
      // Lookup the label. Crash here if bad label.
      let dst_offset = self.labels[&unfixed];

      // Update code cell with special label value
      self.create_jump_destination(dst_offset)
    } else {
      // Update code cell with no-value
      Term::nil()
    }
  }
}
