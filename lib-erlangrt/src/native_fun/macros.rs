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
        $vmvar: &mut crate::emulator::vm::VM,
        $procvar: &mut crate::emulator::process::Process,
        $argsvar: &[LTerm],
      ) -> RtResult<LTerm> {
        crate::native_fun::assert_arity($namestr, $arity, $argsvar);
        define_multiple_args!(
          $namestr, $vmvar, $procvar, $argsvar, 0,
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
///   unused(n) - do nothing
///   usize(n) - take term then unsigned small from it, else return badarg
///   term(n) - take word as a term
///   tuple(n) - the value is a tuple, otherwise badarg
///   list(n), non_empty_list(n) - the value is a list, otherwise badarg
///   atom(n) - must be an atom, otherwise badarg
///   pid(n) - must be a pid, otherwise badarg
///   pid_port(n) - must be a pid or a port, otherwise badarg
///   bool(n) - must be a `true` or `false` atom, otherwise badarg
///
/// Example:
/// ```define_nativefun_args!(vm, curr_p, args, 0,
///   unused(arg1), usize(arg2), term(arg3), list(listarg))```
/// Argument 0 (arg_pos) is auto-increment position counter, should start from 0
macro_rules! define_multiple_args {
  // Empty args are handled here
  ( $fn_name:expr, $vmvar:ident, $procvar:ident, $argsvar:ident, $arg_pos:expr, ) => {};

  // Recurse for multiple args
  ( $fn_name:expr, $vmvar:ident, $procvar:ident, $argsvar:ident, $arg_pos:expr,
    $arg_type:ident $arg_args:tt,
    $($more_args:tt)*
  ) => {
    define_one_arg!( $fn_name, $vmvar, $procvar, $argsvar, $arg_pos, $arg_type $arg_args);
    define_multiple_args!($fn_name, $vmvar, $procvar, $argsvar, ($arg_pos+1), $($more_args)*);
  };
}

macro_rules! define_one_arg {
  // UNUSED args are do-nothing
  ( $fn_name:expr, $vmvar:ident, $procvar:ident, $argsvar:ident, $arg_pos:expr,
    unused($arg_ident:ident)
  ) => {
    // unused $type $arg_ident
  };

  // Term args are just taken as is from memory
  ( $fn_name:expr, $vmvar:ident, $procvar:ident, $argsvar:ident, $arg_pos:expr,
    term($arg_ident:ident)
  ) => {
    let $arg_ident = $argsvar[$arg_pos];
  };

  // Usize args are decoded from term a small unsigned
  ( $fn_name:expr, $vmvar:ident, $procvar:ident, $argsvar:ident, $arg_pos:expr,
    usize($arg_ident:ident)
  ) => {
    let $arg_ident: usize = {
      let tmp = $argsvar[$arg_pos];
      if !(tmp.is_small()) { return crate::fail::create::badarg(); }
      tmp.get_small_unsigned()
    };
  };

  // Tuple args are verified to be a tuple otherwise a badarg is created.
  ( $fn_name:expr, $vmvar:ident, $procvar:ident, $argsvar:ident, $arg_pos:expr,
    tuple($arg_ident:ident)
  ) => {
    let $arg_ident = $argsvar[$arg_pos];
    if !$arg_ident.is_tuple() { return_badarg!($fn_name, $arg_pos, $arg_ident, "tuple"); }
  };

  // List args are verified to be a list or [] otherwise a badarg is created.
  ( $fn_name:expr, $vmvar:ident, $procvar:ident, $argsvar:ident, $arg_pos:expr,
    list($arg_ident:ident)
  ) => {
    let $arg_ident = $argsvar[$arg_pos];
    if !$arg_ident.is_list() { return_badarg!($fn_name, $arg_pos, $arg_ident, "list"); }
  };

  // Non-empty List args are verified to be a list but not [] otherwise a badarg is created.
  ( $fn_name:expr, $vmvar:ident, $procvar:ident, $argsvar:ident, $arg_pos:expr,
    non_empty_list($arg_ident:ident)
  ) => {
    let $arg_ident = $argsvar[$arg_pos];
    if !$arg_ident.is_cons() { return_badarg!($fn_name, $arg_pos, $arg_ident, "non empty list"); }
  };

  // Atom args are verified to be an atom otherwise a badarg is created.
  ( $fn_name:expr, $vmvar:ident, $procvar:ident, $argsvar:ident, $arg_pos:expr,
    atom($arg_ident:ident)
  ) => {
    let $arg_ident = $argsvar[$arg_pos];
    if !$arg_ident.is_atom() { return_badarg!($fn_name, $arg_pos, $arg_ident, "atom"); }
  };

  // Pid args are verified to be a pid or [] otherwise a badarg is created.
  ( $fn_name:expr, $vmvar:ident, $procvar:ident, $argsvar:ident, $arg_pos:expr,
    pid($arg_ident:ident)
  ) => {
    let $arg_ident = $argsvar[$arg_pos];
    if !$arg_ident.is_pid() { return_badarg!($fn_name, $arg_pos, $arg_ident, "pid"); }
  };

  // Atom args are verified to be an atom otherwise a badarg is created.
  ( $fn_name:expr, $vmvar:ident, $procvar:ident, $argsvar:ident, $arg_pos:expr,
    pid_port($arg_ident:ident)
  ) => {
    let $arg_ident = $argsvar[$arg_pos];
    if !$arg_ident.is_pid() && !$arg_ident.is_port()
      { return_badarg!($fn_name, $arg_pos, $arg_ident, "pid|port"); }
  };

  // Atom args are verified to be an atom otherwise a badarg is created.
  ( $fn_name:expr, $vmvar:ident, $procvar:ident, $argsvar:ident, $arg_pos:expr,
    bool($arg_ident:ident)
  ) => {
    let $arg_ident: bool = {
      let tmp = $argsvar[$arg_pos];
      if tmp.is_true() { true }
      else if tmp.is_false() { false }
      else { return_badarg!($fn_name, $arg_pos, tmp, "true|false"); }
    };
  };
}

macro_rules! return_badarg {
  ($fn_name:expr, $arg_pos:expr, $arg_ident:ident, $expected_to_be:expr) => {
    if cfg!(debug_assertions) {
      println!("DBG {}: argument #{} is expected to be a {}, got {}",
        $fn_name, $arg_pos+1, $expected_to_be, $arg_ident);
    }
    return crate::fail::create::badarg();
  };
}