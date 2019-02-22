/// Creates opcode struct and implementation.
/// Struct name follows convention: Opcode<Name><Arity>
/// Body must be a valid body for the run() function returning RtResult<DispatchResult>.
///
/// Example:
///
/// define_opcode!(_vm, ctx, curr_p,
///   name: OpcodeBsStartMatch2,
///   arity: 5,
///   run: {
///     Self::bs_start_match_2(ctx, fail, context)
///   },
///   args: cp_not_nil(fail),
///         load(context),
///         unused(usize_live),
///         unused(term_src),
///         slice(args, 1),
///         load(term_ctxr)
/// );

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
        $vmarg: &mut VM,
        $ctxarg: &mut Context,
        $procarg: &mut Process
      ) -> RtResult<DispatchResult> {
        define_opcode_args!(
          $vmarg, $ctxarg, $procarg, 0,
          $($args)*
        );
        $body
      }
    }
  };
  // end macro impl
}

/// For args, other than unused, create one variable per argument,
/// which will capture each arg from the `ip[$arg_pos]`.
///
/// Arguments can be comma-separated many of:
///   unused(ident) - do nothing
///   usize(ident) - take term then unsigned small from it, debug type check
///   term(ident) - take word as a term
///   load(ident) - take word possibly register or stack cell, and load the value
///   slice(ident,n) - `&[LTerm]` from arg position of length n
///   literal_tuple(ident) - the value is a tuple, which does not need to be
///       "loaded" from a register or stack
///   cp_not_nil(ident) - take a term and assert it is a CP, and not a NIL
///   yreg(ident) - take a term and assert it is an Y register
///
/// Example: define_opcode_args!(vm, ctx, curr_p, 0,
///             unused(arg1), usize(arg2), term(arg3), slice(args,7))
/// Argument 0 (arg_pos) is auto-increment position counter, should start from 0
macro_rules! define_opcode_args {
  ( $vmarg:ident, $ctxarg:ident, $procarg:ident, $arg_pos:expr, ) => {};

  // Use a slice of memory of given size as terms
  ( $vmarg:ident, $ctxarg:ident, $procarg:ident, $arg_pos:expr,
    slice($arg_ident:ident, $slice_sz:expr)
  ) => {
    let $arg_ident = $ctxarg.op_arg_term_slice_at($arg_pos, $slice_sz);
  };

  // UNUSED args are do-nothing
  ( $vmarg:ident, $ctxarg:ident, $procarg:ident, $arg_pos:expr,
    unused($arg_ident:ident)
  ) => {
    // unused $type $arg_ident
  };

  // Term args are just taken as is from memory
  ( $vmarg:ident, $ctxarg:ident, $procarg:ident, $arg_pos:expr,
    term($arg_ident:ident)
  ) => {
    let $arg_ident = $ctxarg.op_arg_read_term_at($arg_pos);
  };

  // Literal Tuple args are ready to use pointers to a tuple, no extra "load"
  // step is required. Only debug check is performed whether the value is
  // a tuple, there will be no check in release.
  ( $vmarg:ident, $ctxarg:ident, $procarg:ident, $arg_pos:expr,
    literal_tuple($arg_ident:ident)
  ) => {
    let $arg_ident = $ctxarg.op_arg_read_term_at($arg_pos).get_tuple_ptr();
  };

  // Usize args are decoded from term a small unsigned
  ( $vmarg:ident, $ctxarg:ident, $procarg:ident, $arg_pos:expr,
    usize($arg_ident:ident)
  ) => {
    let $arg_ident = {
      let tmp = $ctxarg.op_arg_read_term_at($arg_pos);
      debug_assert!(tmp.is_small());
      tmp.get_small_unsigned()
    };
  };

  // Load args are terms which may point to a register or a stack cell
  // and should be loaded before use.
  ( $vmarg:ident, $ctxarg:ident, $procarg:ident, $arg_pos:expr,
    load($arg_ident:ident)
  ) => {
    let $arg_ident = $ctxarg.op_arg_load_term_at($arg_pos, &mut $procarg.heap);
  };

  // Take a term from IP, and assert it is a CP and not a NIL
  ( $vmarg:ident, $ctxarg:ident, $procarg:ident, $arg_pos:expr,
    cp_not_nil($arg_ident:ident)
  ) => {
    let $arg_ident = $ctxarg.op_arg_read_term_at($arg_pos);
    debug_assert!($arg_ident.is_cp() || $arg_ident == LTerm::nil());
  };

  // Take a term from IP, and assert it is a Y register
  ( $vmarg:ident, $ctxarg:ident, $procarg:ident, $arg_pos:expr,
    yreg($arg_ident:ident)
  ) => {
    let $arg_ident = $ctxarg.op_arg_read_term_at($arg_pos);
    debug_assert!($arg_ident.is_regy());
  };

  // Recurse for multiple args
  (
    $vmarg:ident, $ctxarg:ident, $procarg:ident, $arg_pos:expr,
    $arg_type:ident $arg_args:tt,
    $($more_args:tt)*
  ) => {
    define_opcode_args!(
      $vmarg, $ctxarg, $procarg, $arg_pos,
      $arg_type $arg_args
    );
    define_opcode_args!(
      $vmarg, $ctxarg, $procarg, ($arg_pos+1),
      $($more_args)*
    );
  };
}
