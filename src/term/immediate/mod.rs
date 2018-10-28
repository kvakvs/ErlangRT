////!
////! Low level term representation for compact heap storage
////!
////! Term bits are: `.... .... ..bb aaPP`
////!
////! Here "PP" are the primary tag, one of `primary_tag::TAG_IMMED`
////! And "aa" with size 2 bits, uses `Immediate1` bits.
////!
////! To use `Immediate2` bits set "aa" to `TAG_IMM1_IMM2` and set "bb" to the
////!    desired value from `Immediate2` enum.
//
//mod imm1;
//mod imm2;
//mod imm3;
//
//use rt_defs;
//use rt_defs::{Word, SWord};
//
//pub use self::imm1::*;
//pub use self::imm2::*;
//pub use self::imm3::*;
//
//
////
//// Construction
////
//
///// Create a raw value for a term from atom index
//#[inline]
//pub fn make_atom_raw(val: Word) -> Word {
//  combine_imm2_prefix_and_val(val, IMM2_ATOM_PREFIX)
//}
//
//
///// Same as `make_atom_raw` but compile-time for predefined atoms table
//#[inline]
//pub const fn make_atom_raw_const(val: Word) -> Word {
//  combine_imm2_prefix_and_val_const(val, IMM2_ATOM_PREFIX)
//}
//
//
///// Create a raw value for a pid term from process index
//#[inline]
//pub fn make_pid_raw(pindex: Word) -> Word {
//  combine_imm1_prefix_and_val(pindex, IMM1_PID_PREFIX)
//}
//
//
///// Create a raw smallint value for a term from atom index
//#[inline]
//pub fn make_small_raw(val: SWord) -> Word {
//  combine_imm1_prefix_and_val_signed(val, IMM1_SMALL_PREFIX)
//}
//
//
//#[inline]
//pub fn make_xreg_raw(x: Word) -> Word {
//  assert!(x < rt_defs::MAX_XREGS);
//  create_imm3(x, IMM3_XREG_PREFIX)
//}
//
//
//#[inline]
//pub fn make_yreg_raw(x: Word) -> Word {
//  create_imm3(x, IMM3_YREG_PREFIX)
//}
//
//
//#[inline]
//pub fn make_fpreg_raw(x: Word) -> Word {
//  assert!(x < rt_defs::MAX_FPREGS);
//  create_imm3(x, IMM3_FPREG_PREFIX)
//}
//
////#[inline]
////pub fn make_label_raw(x: Word) -> Word {
////  create_imm3(x, IMM3_LABEL_PREFIX)
////}
//
////
//// Checks
////
//
//#[inline]
//pub fn is_pid_raw(val: Word) -> bool {
//  get_imm1_prefix(val) == IMM1_PID_PREFIX
//}
//
///// Check whether a Word represents an atom.
//#[inline]
//pub fn is_atom_raw(val: Word) -> bool {
//  get_imm2_prefix_and_tag(val) == IMM2_ATOM_PREFIX
//}
//
///// Check whether a Word represents a small integer.
//#[inline]
//pub fn is_small_raw(val: Word) -> bool {
//  get_imm1_prefix_and_tag(val) == IMM1_SMALL_PREFIX
//}
//
//
////
//// Testing section
////
//
//#[cfg(test)]
//mod tests {
//  use super::*;
//  use term::primary;
//
//  #[test]
//  fn test_imm3_tags() {
//    let n = IMM3_PREFIX;
//    assert_eq!(primary::get_tag(n), primary::TAG_IMMED);
//    assert_eq!(get_imm1_tag(n), TAG_IMM1_IMM2);
//    assert_eq!(get_imm2_tag(n), TAG_IMM2_IMM3);
//  }
//
//  fn test_imm3(check_val: Word) {
//    let n = create_imm3(check_val, IMM3_XREG_PREFIX);
//    assert_eq!(primary::get_tag(n), primary::TAG_IMMED);
//    assert_eq!(get_imm1_tag(n), TAG_IMM1_IMM2);
//    assert_eq!(get_imm2_tag(n), TAG_IMM2_IMM3);
//    assert_eq!(get_imm3_tag(n), TAG_IMM3_XREG);
//    assert_eq!(get_imm3_prefix(n), IMM3_PREFIX);
//    assert_eq!(get_imm3_value(n), check_val);
//  }
//
//  #[test]
//  fn test_imm3_new() {
//    test_imm3(0usize);
//    test_imm3(0b1000000usize);
//
//    // imm3 with all other tags consumes 8 bits, so try some special values
//    // For 32-bit system, try a 24-bit value to see that the bits are not eaten
//    test_imm3(0x100002usize);
//    if cfg!(target_pointer_width = "64") {
//      // Try a 56 bit value
//      test_imm3(0x10000000000002usize);
//    }
//  }
//}
