/// Format a message if `trace_beam_loader` feature is enabled.
#[macro_export]
macro_rules! rtdbg {
  ($($arg:tt)*) => (if cfg!(feature = "trace_beam_loader") {
    print!("{}", module());
    println!($($arg)*);
  })
}
