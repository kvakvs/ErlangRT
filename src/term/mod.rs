//!
//! Implements tagged term value, which uses some bits to define its type,
//! can store immediate value or a pointer to process heap.
//!
pub mod immediate;
pub mod friendly;
pub mod low_level;
pub mod primary_tag;
