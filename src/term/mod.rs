//!
//! Implements tagged term value, which uses some bits to define its type,
//! can store immediate value or a pointer to process heap.
//!
pub mod fterm;
pub mod immediate;
pub mod integral;
pub mod lterm;
pub mod primary;
pub mod raw;
