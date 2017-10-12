use defs::Word;
use term::lterm::LTerm;
use beam::gen_op;
use emulator::code::{opcode, Labels, RefCode};

/// Print to screen disassembly of the current function.
#[allow(dead_code)]
pub unsafe fn disasm(code: RefCode, _labels: Option<&Labels>) {
  let mut ip = &code[0] as *const Word;
  let iend = ip.offset(code.len() as isize);

  while ip < iend {
    let op = opcode::from_memory_word(*ip);
    assert!(op < gen_op::OPCODE_MAX);
    print!("0x{:p}: {} ", ip, gen_op::opcode_name(op as u8));
    ip = ip.offset(1);

    let n_args = gen_op::opcode_arity(op as u8) as Word;

    for arg_index in 0..n_args {

      let arg_raw = *ip.offset(arg_index as isize);
      let arg = LTerm::from_raw(arg_raw);

      // Header value in code marks an embedded block of terms
      // Header{Arity=3} Term1 Term2 Term3
      if arg.is_header() {
        print!("[");
        for _h in 0..arg.header_arity() {
          print!("{} ", LTerm::from_raw(*ip.offset(arg_index as isize + 1)));
          ip = ip.offset(1);
        }
        print!("] ");
      } else { // Otherwise it is printable like this, and occupies 1w
        print!("{} ", arg)
      }
    }

    ip = ip.offset(n_args as isize);
    println!();
  }
}
