//! Term package implements two types of Erlang values:
//!
//! * `Term` (or a Term value) - a tagged machine-word-sized term, which uses
//!     some bits to define its type, and can store either values or
//!     pointers to heap
//!
//! As well as operations on terms, such as arithmetic or comparisons.
pub mod arith; // simple operations on numeric terms
pub mod boxed;
pub mod builders; // simple term builder helpers
pub mod classify; // term ordering (for comparisons)
pub mod compare; // term comparisons (less, equal, greater)
pub mod integral; // integral value (small or bignum) for fterms
pub mod term_builder; /* implements ITermBuilder for RT VM // term in memory (dynamic runtime dispatch) */
pub mod value; // Value stored in one machine word
