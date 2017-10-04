pub mod code_srv;
pub mod function;
pub mod module;
pub mod process;
pub mod vm;

pub mod mfa;
pub mod funarity;

// Generated modules - create by calling `make codegen` in the root directory
// or by invoking `make` in the `codegen/` directory
pub mod gen_op;
