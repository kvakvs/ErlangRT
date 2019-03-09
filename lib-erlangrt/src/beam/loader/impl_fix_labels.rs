use crate::{
  beam::loader::{LoaderState, PatchLocation},
  defs,
  emulator::code::LabelId,
  fail::RtResult,
  term::{boxed, lterm::Term},
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
          self.code[cmd_offset] = self.postprocess_fix_1_label(val)
        }

        PatchLocation::PatchJtabElement(jtab, index) => {
          let jtab_ptr = jtab.get_box_ptr_mut::<boxed::Tuple>();
          unsafe {
            let val = (*jtab_ptr).get_element(index);
            (*jtab_ptr).set_element_raw(index, self.postprocess_fix_1_label(val))
          }
        }
      } // match
    } // for ploc
    Ok(())
  }


  /// Helper for `postprocess_fix_label`, takes a word from code memory or from
  /// a jump table, resolves as if it was a label index, and returns a value
  /// to be put back into memory.
  fn postprocess_fix_1_label(&self, val: Term) -> defs::Word {
    // Convert from Term smallint to integer and then to labelid
    let unfixed = val.get_small_signed() as defs::Word;

    // Zero label id means no location, so we will store NIL [] there
    if unfixed > 0 {
      let unfixed_l = LabelId(unfixed);

      // Lookup the label. Crash here if bad label.
      let dst_offset = self.labels[&unfixed_l];

      // Update code cell with special label value
      self.create_jump_destination(dst_offset)
    } else {
      // Update code cell with no-value
      Term::nil().raw()
    }
  }
}
