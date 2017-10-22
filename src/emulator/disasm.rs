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
    ip = disasm_op(ip);
  }
}


/// Given an IP code pointer which points to the opcode - print the opcode and
/// args. Returns updated IP which points at the next opcode.
pub unsafe fn disasm_op(ip0: *const Word) -> *const Word {
  let mut ip = ip0;

  let op = opcode::from_memory_word(*ip);
  assert!(op < gen_op::OPCODE_MAX);

  print!("{:p}: {} ", ip, gen_op::opcode_name(op as u8));
  ip = ip.offset(1);

  let n_args = gen_op::opcode_arity(op as u8) as Word;

  for arg_index in 0..n_args {

    let arg_raw = *ip.offset(arg_index as isize);
    let arg = LTerm::from_raw(arg_raw);

    print!("{} ", arg)
  }

  println!();

  ip.offset(n_args as isize)
}

