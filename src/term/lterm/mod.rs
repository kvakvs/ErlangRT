//! Do not import submodules directly, use `use term::lterm::*;` instead.

mod aspect_atom;
mod aspect_binary;
mod aspect_boxed;
mod aspect_cp;
mod aspect_export;
mod aspect_float;
mod aspect_fun;
mod aspect_list;
mod aspect_map;
mod aspect_pid;
mod aspect_port;
mod aspect_reference;
mod aspect_smallint;
mod aspect_tuple;
mod format;
mod lterm_impl;


//pub use term::lterm::aspect_atom::*;
//pub use term::lterm::aspect_binary::*;
//pub use term::lterm::aspect_boxed::*;
//pub use term::lterm::aspect_cp::*;
//pub use term::lterm::aspect_export::*;
//pub use term::lterm::aspect_float::*;
//pub use term::lterm::aspect_fun::*;
//pub use term::lterm::aspect_list::*;
//pub use term::lterm::aspect_map::*;
//pub use term::lterm::aspect_pid::*;
//pub use term::lterm::aspect_port::*;
//pub use term::lterm::aspect_reference::*;
//pub use term::lterm::aspect_smallint::*;
//pub use term::lterm::aspect_tuple::*;

pub use crate::term::lterm::lterm_impl::*;
