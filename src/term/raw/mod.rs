//pub mod rbignum;
pub mod heapobj;
pub mod ho_bignum;
pub mod ho_binary;
pub mod ho_import;
pub mod rcons;
pub mod rtuple;

//pub use term::raw::rbignum::BignumPtr;

pub use term::raw::rcons::ConsPtr;
pub use term::raw::rcons::ConsPtrMut;

pub use term::raw::rtuple::TuplePtr;
pub use term::raw::rtuple::TuplePtrMut;
