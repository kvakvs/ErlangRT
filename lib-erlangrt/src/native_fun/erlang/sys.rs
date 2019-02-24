use crate::{
  defs::exc_type::ExceptionType,
  emulator::{gen_atoms, process::Process},
  fail::{RtErr, RtResult},
  term::{builders::make_badfun_n, lterm::LTerm},
};

#[allow(dead_code)]
fn module() -> &'static str {
  "native funs module for erlang[sys]: "
}

/// Create an error for a NIF not loaded/not implemented.
define_nativefun!(_vm, proc, args,
  name: "erlang:nif_error/1", struct_name: NfErlangNifError1, arity: 1,
  invoke: {
    Err(RtErr::Exception(
      ExceptionType::Error, make_badfun_n(args, &mut proc.heap)?
    ))
  },
  args:
);

/// Create an error for a NIF not loaded/not implemented.
define_nativefun!(_vm, proc, args,
  name: "erlang:nif_error/2", struct_name: NfErlangNifError2, arity: 2,
  invoke: {
    Err(RtErr::Exception(
      ExceptionType::Error, make_badfun_n(args, &mut proc.heap)?
    ))
  },
  args:
);

/// Create an exception of type `error` with an argument.
define_nativefun!(_vm, proc, args,
  name: "erlang:error/2", struct_name: NfErlangError2, arity: 2,
  invoke: { error_2(proc, reason, err_args) },
  args: term(reason), term(err_args),
);

pub fn error_2(proc: &mut Process, reason: LTerm, err_args: LTerm) -> RtResult<LTerm> {
  let tuple_val = proc.heap.tuple2(reason, err_args)?;
  Err(RtErr::Exception(ExceptionType::Error, tuple_val))
}

/// Create an exception of type `error`.
define_nativefun!(_vm, _proc, args,
  name: "erlang:error/1", struct_name: NfErlangError1, arity: 1,
  invoke: { Err(RtErr::Exception(ExceptionType::Error, reason)) },
  args: term(reason),
);

/// Make a nice face like we are loading something here
// TODO: Implement pre-linked NIF modules which are ready to be activated
define_nativefun!(_vm, _proc, args,
  name: "erlang:load_nif/2", struct_name: NfErlangLoadNif2, arity: 2,
  invoke: {
    println!("load_nif({}, {}) - doing nothing", path, load_info);
    Ok(gen_atoms::OK)
  },
  args: list(path), term(load_info),
);
