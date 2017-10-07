use defs::Word;
use term::lterm::LTerm;
use beam::gen_op;
use emulator::code::{Code, Labels};

/// Print to screen disassembly of the current function.
#[allow(dead_code)]
pub fn disasm(code: &Code, _labels: Option<&Labels>) {
  let mut i = 0;
  while i < code.len() {
    let op = code[i];
    assert!(op < gen_op::OPCODE_MAX);
    print!("0x{:04x} {} ", i, gen_op::opcode_name(op as u8));
    i += 1;

    let arity = gen_op::opcode_arity(op as u8) as Word;
    for j in 0..arity {
      let arg_raw = code[i + j];
      let arg = LTerm::from_raw(arg_raw);

      // Header value in code marks an embedded block of terms
      // Header{Arity=3} Term1 Term2 Term3
      if arg.is_header() {
        print!("[");
        for _h in 0..arg.header_arity() {
          print!("{} ", LTerm::from_raw(code[i + j + 1]));
          i += 1;
        }
        print!("] ");
      } else { // Otherwise it is printable like this, and occupies 1w
        print!("{} ", arg)
      }
    }

    i += arity;
    println!();
  }
}
