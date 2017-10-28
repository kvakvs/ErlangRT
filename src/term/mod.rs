//!
//! Implements two types of Erlang values:
//!
//! * FTerm (or Friendly Term) - a typesafe Rust enum which represents most of
//!     possible values for load-time processing.
//! * LTerm (or a low level Term) - a tagged term Word, which uses some bits to
//!     define its type, and can store either just values or pointers to heap.
//!
//! As well as operations on terms, such as arithmetic or comparisons.
//!
pub mod compare;
pub mod fterm;
pub mod immediate;
pub mod integral;
pub mod lterm;
pub mod primary;
pub mod raw;
