//! Generated by `codegen/create_gen_atoms.py`
//! Creates array of predefined atoms
//! Config used: OTP20
#![allow(dead_code)]

use term::immediate::{make_atom_raw_const};
use term::lterm::LTerm;


pub const SYM_PLUS: LTerm = LTerm { value: make_atom_raw_const(0) };
pub const SYM_MINUS: LTerm = LTerm { value: make_atom_raw_const(1) };
pub const APPLY: LTerm = LTerm { value: make_atom_raw_const(2) };
pub const BADARG: LTerm = LTerm { value: make_atom_raw_const(3) };
pub const BADARITH: LTerm = LTerm { value: make_atom_raw_const(4) };
pub const BADARITY: LTerm = LTerm { value: make_atom_raw_const(5) };
pub const BADFUN: LTerm = LTerm { value: make_atom_raw_const(6) };
pub const BADMATCH: LTerm = LTerm { value: make_atom_raw_const(7) };
pub const CASE_CLAUSE: LTerm = LTerm { value: make_atom_raw_const(8) };
pub const ERLANG: LTerm = LTerm { value: make_atom_raw_const(9) };
pub const ERROR: LTerm = LTerm { value: make_atom_raw_const(10) };
pub const EXIT: LTerm = LTerm { value: make_atom_raw_const(11) };
pub const FALSE: LTerm = LTerm { value: make_atom_raw_const(12) };
pub const FUNCTION_CLAUSE: LTerm = LTerm { value: make_atom_raw_const(13) };
pub const HIGH: LTerm = LTerm { value: make_atom_raw_const(14) };
pub const IF_CLAUSE: LTerm = LTerm { value: make_atom_raw_const(15) };
pub const INIT: LTerm = LTerm { value: make_atom_raw_const(16) };
pub const KILL: LTerm = LTerm { value: make_atom_raw_const(17) };
pub const KILLED: LTerm = LTerm { value: make_atom_raw_const(18) };
pub const LOW: LTerm = LTerm { value: make_atom_raw_const(19) };
pub const NOCATCH: LTerm = LTerm { value: make_atom_raw_const(20) };
pub const NORMAL: LTerm = LTerm { value: make_atom_raw_const(21) };
pub const OK: LTerm = LTerm { value: make_atom_raw_const(22) };
pub const SELF: LTerm = LTerm { value: make_atom_raw_const(23) };
pub const SYSTEM_LIMIT: LTerm = LTerm { value: make_atom_raw_const(24) };
pub const THROW: LTerm = LTerm { value: make_atom_raw_const(25) };
pub const TRAP_EXIT: LTerm = LTerm { value: make_atom_raw_const(26) };
pub const TRUE: LTerm = LTerm { value: make_atom_raw_const(27) };
pub const UNDEF: LTerm = LTerm { value: make_atom_raw_const(28) };
pub const UNDEFINED: LTerm = LTerm { value: make_atom_raw_const(29) };

pub static ATOM_INIT_NAMES: &'static [&'static str] = &[
  "+", // id=0
  "-", // id=1
  "apply", // id=2
  "badarg", // id=3
  "badarith", // id=4
  "badarity", // id=5
  "badfun", // id=6
  "badmatch", // id=7
  "case_clause", // id=8
  "erlang", // id=9
  "error", // id=10
  "exit", // id=11
  "false", // id=12
  "function_clause", // id=13
  "high", // id=14
  "if_clause", // id=15
  "init", // id=16
  "kill", // id=17
  "killed", // id=18
  "low", // id=19
  "nocatch", // id=20
  "normal", // id=21
  "ok", // id=22
  "self", // id=23
  "system_limit", // id=24
  "throw", // id=25
  "trap_exit", // id=26
  "true", // id=27
  "undef", // id=28
  "undefined", // id=29
];
