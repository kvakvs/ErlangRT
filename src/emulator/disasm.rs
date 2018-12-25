use crate::{
  beam::gen_op,
  defs::Word,
  emulator::{
    code::{opcode, CodePtr, Labels, RefCode},
    code_srv::CodeServer,
  },
  term::lterm::*,
};
use colored::*;

/// Print to screen disassembly of the current function.
#[allow(dead_code)]
pub unsafe fn disasm(code: RefCode, _labels: Option<&Labels>, code_server: &CodeServer) {
  let mut ip = &code[0] as *const Word;
  let iend = ip.add(code.len());

  while ip < iend {
    ip = disasm_op(ip, code_server);
  }
}

/// Given an IP code pointer which points to the opcode - print the opcode and
/// args. Returns updated IP which points at the next opcode.
pub unsafe fn disasm_op(ip0: *const Word, code_server: &CodeServer) -> *const Word {
  let mut ip = ip0;

  let op = opcode::from_memory_ptr(ip);
  assert!(
    op <= gen_op::OPCODE_MAX,
    "Opcode {:?} is too big, more than max {:?}",
    op,
    gen_op::OPCODE_MAX
  );

  if let Some(mfa) = code_server.code_reverse_lookup(CodePtr::new(ip)) {
    print!("{} ", format!("{}", mfa).cyan());
  }

  print!(
    "{}: {} ",
    format!("{:p}", ip).bright_black(),
    gen_op::opcode_name(op).white()
  );
  ip = ip.offset(1);

  let n_args = gen_op::opcode_arity(op) as Word;
  disasm_op_args(ip, n_args);

  println!();

  ip.add(n_args)
}

unsafe fn disasm_op_args(ip: *const Word, n_args: Word) {
  for arg_index in 0..n_args {
    let arg = core::ptr::read(ip.add(arg_index) as *const LTerm);

    print!("{}", arg);
    if arg_index < n_args - 1 {
      print!("{}", ", ".red())
    }
  }
}
