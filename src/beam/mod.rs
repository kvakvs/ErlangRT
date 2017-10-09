//! Collection of modules handling BEAM file format and BEAM instructions
pub mod loader;
pub mod compact_term;
pub mod vm_loop;

// Generated modules - create by calling `make codegen` in the root directory
// or by invoking `make` in the `codegen/` directory
pub mod gen_op; // generated
pub mod vm_dispatch; // generated
pub mod opcodes; // generated
