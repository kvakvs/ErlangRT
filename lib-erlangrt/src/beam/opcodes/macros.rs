/// Creates opcode struct and implementation.
/// Struct name follows convention: `Opcode<Name><Arity>`
/// Body must be a valid body for the run() function returning
/// `RtResult<DispatchResult>`.
///
/// The output is:
/// ```
/// struct StructName {}
/// impl StructName {
///   #[inline]
///   pub fn __run(...) {
///     let var = args[...] + argument type checks
///     $body
///   }
/// }
/// ```
///
/// Example:
/// ```
/// define_opcode!(_vm, ctx, curr_p,
///   name: OpcodeBsStartMatch2,
///   arity: 5,
///   run: {
///     Self::bs_start_match_2(ctx, fail, context)
///   },
///   args: cp_or_nil(fail), load(context), IGNORE(usize_live), IGNORE(term_src),
///         slice(args, 1), load(term_ctxr));
/// ```

#[macro_export]
macro_rules! define_opcode {
  (
    $vmarg:ident, $ctxarg:ident, $procarg:ident,
    name: $struct_name:ident,
    arity: $arity:expr,
    run: $body:block,
    args: $($args:tt)*
  ) => {
    pub struct $struct_name {}
    impl $struct_name {
      pub const ARITY: usize = $arity;

      #[inline]
      pub fn __run(
        $vmarg: &mut crate::emulator::vm::VM,
        $ctxarg: &mut Context,
        $procarg: &mut Process
      ) -> RtResult<DispatchResult> {
        fetch_multiple_args!(
          $vmarg, $ctxarg, $procarg, 0,
          $($args)*
        );
        $body
      }
    }
  };
  // end macro impl
}

/// For args, other than unused, creates one local variable per argument,
/// which will capture each arg from the `ip[$arg_pos]`.
///
/// Arguments can be comma-separated many of:
///   IGNORE(n) - do nothing
///   usize(n) - take term then unsigned small from it, debug type check
///   load_usize(n) - first load then treat it as a small unsigned
///   term(n) - take word as a term
///   load(n) - take word possibly register or stack cell, and load the value
///   slice(ident,n) - `&[Term]` from arg position of length n
///   literal_tuple(n) - the value is a tuple, which does not need to be
///       "loaded" from a register or stack
///   literal_jumptable(n) - the value is a jumptable (special tuple with pairs)
///   cp_or_nil(n) - take a term and assert it is either a CP, or a NIL
///   yreg(n) - take a term and assert it is an Y register
///   binary_match_state(n) - extract and assert the boxed is a binary match state
///
/// Example:
/// ```define_opcode_args!(vm, ctx, curr_p, 0,
///   IGNORE(arg1), usize(arg2), term(arg3), slice(args,7))```
/// Argument 0 (arg_pos) is auto-increment position counter, should start from 0
macro_rules! fetch_multiple_args {
  // Empty args are handled here
  ( $vmarg:ident, $ctxarg:ident, $procarg:ident, $arg_pos:expr, ) => {};

  // Handle the special type for arg: slice() which will step arg_pos by arity
  // instead of stepping by 1. Also recurse for more args.
  // Use a slice of memory of given size as terms.
  ( $vmarg:ident, $ctxarg:ident, $procarg:ident, $arg_pos:expr,
    slice($arg_ident:ident, $slice_sz:expr),
    $($more_args:tt)*
  ) => {
    let $arg_ident = $ctxarg.op_arg_term_slice_at($arg_pos, $slice_sz);
    fetch_multiple_args!($vmarg, $ctxarg, $procarg, ($arg_pos+$slice_sz), $($more_args)*);
  };

  // Recurse for multiple args
  ( $vmarg:ident, $ctxarg:ident, $procarg:ident, $arg_pos:expr,
    $arg_type:ident $arg_args:tt,
    $($more_args:tt)*
  ) => {
    fetch_one_arg!($vmarg, $ctxarg, $procarg, $arg_pos, $arg_type $arg_args);
    fetch_multiple_args!($vmarg, $ctxarg, $procarg, ($arg_pos+1), $($more_args)*);
  };
}

macro_rules! fetch_one_arg {
  // UNUSED args are do-nothing
  (
    $vmarg:ident, $ctxarg:ident, $procarg:ident, $arg_pos:expr,
    IGNORE($arg_ident:ident)
  ) => {
    // unused $type $arg_ident
  };

  // Term args are just taken as is from memory
  (
    $vmarg:ident, $ctxarg:ident, $procarg:ident, $arg_pos:expr,
    term($arg_ident:ident)
  ) => {
    let $arg_ident = $ctxarg.op_arg_read_term_at($arg_pos);
  };

  // Literal Tuple args are ready to use pointers to a tuple, no extra "load"
  // step is required. Only debug check is performed whether the value is
  // a tuple, there will be no check in release.
  (
    $vmarg:ident, $ctxarg:ident, $procarg:ident, $arg_pos:expr,
    literal_tuple($arg_ident:ident)
  ) => {
    let $arg_ident = $ctxarg.op_arg_read_term_at($arg_pos).get_tuple_ptr();
  };

  // Literal Jumptable args are ready to use pointers to a jumptable
  (
    $vmarg:ident, $ctxarg:ident, $procarg:ident, $arg_pos:expr,
    literal_jumptable($arg_ident:ident)
  ) => {
    let $arg_ident = $ctxarg
      .op_arg_read_term_at($arg_pos)
      .get_box_ptr::<crate::term::boxed::JumpTable>();
  };

  // Usize args are decoded from term a small unsigned
  (
    $vmarg:ident, $ctxarg:ident, $procarg:ident, $arg_pos:expr,
    usize($arg_ident:ident)
  ) => {
    let $arg_ident = {
      let tmp = $ctxarg.op_arg_read_term_at($arg_pos);
      debug_assert!(tmp.is_small(), "Expected a small int, got {}", tmp);
      tmp.get_small_unsigned()
    };
  };

  // Load_usize args are first loaded then decoded from term a small unsigned
  (
    $vmarg:ident, $ctxarg:ident, $procarg:ident, $arg_pos:expr,
    load_usize($arg_ident:ident)
  ) => {
    let $arg_ident = {
      let tmp = $ctxarg.op_arg_load_term_at($arg_pos, &mut $procarg.heap);
      debug_assert!(tmp.is_small(), "Expected a small int, got {}", tmp);
      tmp.get_small_unsigned()
    };
  };

  // Load args are terms which may point to a register or a stack cell
  // and should be loaded before use.
  (
    $vmarg:ident, $ctxarg:ident, $procarg:ident, $arg_pos:expr,
    load($arg_ident:ident)
  ) => {
    let $arg_ident = $ctxarg.op_arg_load_term_at($arg_pos, &mut $procarg.heap);
  };

  // Take a term from IP, and assert it is a CP or a NIL
  (
    $vmarg:ident, $ctxarg:ident, $procarg:ident, $arg_pos:expr,
    cp_or_nil($arg_ident:ident)
  ) => {
    let $arg_ident = $ctxarg.op_arg_read_term_at($arg_pos);
    debug_assert!($arg_ident.is_cp() || $arg_ident == Term::nil());
  };

  // Take a term from IP, and assert it is a Y register
  (
    $vmarg:ident, $ctxarg:ident, $procarg:ident, $arg_pos:expr,
    yreg($arg_ident:ident)
  ) => {
    let $arg_ident = $ctxarg.op_arg_read_term_at($arg_pos);
    debug_assert!(
      $arg_ident.is_register_y(),
      "Expected an Y register, got {}",
      $arg_ident
    );
  };

  // Take a term from IP, and assert it is a binary match state
  (
    $vmarg:ident, $ctxarg:ident, $procarg:ident, $arg_pos:expr,
    binary_match_state($arg_ident:ident)
  ) => {
    let $arg_ident = {
      let tmp = $ctxarg.op_arg_load_term_at($arg_pos, &mut $procarg.heap);
      debug_assert!(
        tmp.is_boxed_of_type(crate::term::boxed::BOXTYPETAG_BINARY_MATCH_STATE),
        "Expected a binary match state, got {}",
        tmp
      );
      tmp.get_box_ptr_mut::<crate::term::boxed::binary::match_state::BinaryMatchState>()
    };
  };
}
