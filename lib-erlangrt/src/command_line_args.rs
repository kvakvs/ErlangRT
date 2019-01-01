use core::iter;
use std::collections::HashMap;

#[derive(Debug)]
pub enum DistMode {
  ShortName,
  FullName,
}

/// Arguments to start Erlang VM. Build your own, or parse from a string.
/// Parsing more than once will override the existing values allowing you to
/// combine multiple sources of args such as command line, vmargs file, OS env
/// variable and so on.
#[derive(Debug)]
pub struct ErlStartArgs {
  /// For unknown args, this is key-value arg store. Known args have their own
  /// variables
  dict: HashMap<String, Vec<String>>,
  pub node_name: String,
  pub dist_mode: DistMode,
  /// Which modules:functions to start (option -s m f arg1,...)
  pub start: Vec<Vec<String>>,
}

impl ErlStartArgs {
  pub fn new() -> Self {
    Self {
      dict: HashMap::new(),
      node_name: "nonode@nohost".to_string(),
      dist_mode: DistMode::ShortName,
      start: Vec::new(),
    }
  }

  pub fn add_start(&mut self, data: &[&str]) {
    self.start.push(data.iter().map(|s| s.to_string()).collect())
  }

  /// Copy args from any string iterator source
  pub fn populate_with<ITER>(&mut self, mut iter: ITER)
  where
    ITER: Iterator<Item = String>,
  {
    loop {
      if let Some(s) = iter.next() {
        self.arg1(s.as_ref())
      } else {
        break;
      }
    }
  }

  /// Parses and adds one argument with no parameter
  pub fn arg1(&mut self, arg_val: &str) {
    self.arg(arg_val, iter::empty());
  }

  /// Parses and adds one argument with possible parameters which will be
  /// fetched from the mutable string-collection iterator
  pub fn arg<ITER>(&mut self, _arg_val: &str, _more_args: ITER)
  where
    ITER: Iterator<Item = String>,
  {

  }
}
