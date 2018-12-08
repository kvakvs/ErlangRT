//!
//! Term package implements two types of Erlang values:
//!
//! * `FTerm` (or Friendly Term) - a typesafe Rust enum which represents most of
//!     possible values for load-time processing.
//! * `LTerm` (or a low level Term) - a tagged term Word, which uses some bits
//!     to define its type, and can store either just values or pointers to heap
//!
//! As well as operations on terms, such as arithmetic or comparisons.
//!
pub mod arith; // simple operations on numeric terms
pub mod boxed;
pub mod builders; // simple term builder helpers
pub mod classify; // term ordering (for comparisons)
pub mod compare; // term comparisons (less, equal, greater)
pub mod fterm; // friendly term as Rust enum
pub mod immediate; // immediate term value, subset of lterm
pub mod integral; // integral value (small or bignum) for fterms
pub mod lterm; // low level packed term
pub mod primary; // primary tag and header tag handling for packed lterms
pub mod raw; // access helpers for some raw heap structures
pub mod term_builder; // implements ITermBuilder for RT VM // term in memory (dynamic runtime dispatch)
