/// Define a new native function header, along with simple argument checks.
/// Note: if the body arg contains a call to the implementation, it is in your
/// best interest to have that impl `#[inline]`.
///
/// The output is:
/// ```
/// struct StructName {}
/// impl StructName {
///   pub fn _f(...) {
///     let var = args[...] + argument type checks
///     $body
///   }
/// }
/// ```
///
/// From here you should pass the actual used data further to the implementation.
/// Usage:
/// ```
/// define_nativefun!(vm, proc, args,
///   name: "lists:member/3", struct_name: NativeListsMember, arity: 3,
///   invoke: {},
///   args: term(key), usize(pos), term(list)
/// )
/// ```
#[macro_export]
macro_rules! define_nativefun {
  (
    $vmvar:ident, $procvar:ident, $argsvar:ident,
    name: $namestr:expr, struct_name: $struct_name:ident,
    arity: $arity:expr,
    invoke: $body:block,
    args: $($args:tt)*
  ) => {
    pub struct $struct_name {}
    impl $struct_name {
      pub fn _f(
        $vmvar: &mut VM,
        $procvar: &mut Process,
        $argsvar: &[LTerm],
      ) -> RtResult<LTerm> {
        crate::native_fun::assert_arity($namestr, $arity, $argsvar);
        define_nativefun_args!(
          $vmvar, $procvar, $argsvar, 0, $($args)*
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
///   unused(ident) - do nothing
///   usize(ident) - take term then unsigned small from it, else return badarg
///   term(ident) - take word as a term
///   tuple(ident) - the value is a tuple, otherwise returns badarg
///   list(ident) - the value is a list, otherwise returns badarg
///
/// Example:
/// ```define_nativefun_args!(vm, curr_p, args, 0,
///   unused(arg1), usize(arg2), term(arg3), list(listarg))```
/// Argument 0 (arg_pos) is auto-increment position counter, should start from 0
macro_rules! define_nativefun_args {
  // Empty args are handled here
  ( $vmvar:ident, $procvar:ident, $argsvar:ident, $arg_pos:expr, ) => {};

  // UNUSED args are do-nothing
  ( $vmvar:ident, $procvar:ident, $argsvar:ident, $arg_pos:expr,
    unused($arg_ident:ident)
  ) => {
    // unused $type $arg_ident
  };

  // Term args are just taken as is from memory
  ( $vmvar:ident, $procvar:ident, $argsvar:ident, $arg_pos:expr,
    term($arg_ident:ident)
  ) => {
    let $arg_ident = $argsvar[$arg_pos];
  };

  // Tuple args are verified to be a tuple otherwise a badarg is created.
  ( $vmvar:ident, $procvar:ident, $argsvar:ident, $arg_pos:expr,
    tuple($arg_ident:ident)
  ) => {
    let $arg_ident = $argsvar[$arg_pos];
    if !$arg_ident.is_tuple() { return fail::create::badarg(); }
  };

  // List args are verified to be a list or [] otherwise a badarg is created.
  ( $vmvar:ident, $procvar:ident, $argsvar:ident, $arg_pos:expr,
    list($arg_ident:ident)
  ) => {
    let $arg_ident = $argsvar[$arg_pos];
    if !$arg_ident.is_list() { return fail::create::badarg(); }
  };

  // Usize args are decoded from term a small unsigned
  ( $vmvar:ident, $procvar:ident, $argsvar:ident, $arg_pos:expr,
    usize($arg_ident:ident)
  ) => {
    let $arg_ident = {
      let tmp = $argsvar[$arg_pos];
      if !(tmp.is_small()) { return fail::create::badarg(); }
      tmp.get_small_unsigned()
    };
  };

  // Recurse for multiple args
  (
    $vmvar:ident, $procvar:ident, $argsvar:ident, $arg_pos:expr,
    $arg_type:ident $arg_args:tt,
    $($more_args:tt)*
  ) => {
    define_nativefun_args!(
      $vmvar, $procvar, $argsvar, $arg_pos,
      $arg_type $arg_args
    );
    define_nativefun_args!(
      $vmvar, $procvar, $argsvar, ($arg_pos+1),
      $($more_args)*
    );
  };
}
