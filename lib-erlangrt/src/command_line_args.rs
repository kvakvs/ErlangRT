#[derive(Debug)]
pub enum NodeName {
  Short(String),
  Full(String),
}

/// Arguments to start Erlang VM. Build your own, or parse from a string.
/// Parsing more than once will override the existing values allowing you to
/// combine multiple sources of args such as command line, vmargs file, OS env
/// variable and so on.
#[derive(Debug)]
pub struct ErlStartArgs {
  /// Storage for other unknown args
  other_args: Vec<String>,
  pub node: NodeName,
  /// Which modules:functions to start (option -s m f arg1,...)
  pub start: Vec<Vec<String>>,
  pub search_path: Vec<String>,
}

impl ErlStartArgs {
  pub fn new() -> Self {
    Self {
      other_args: Vec::new(),
      node: NodeName::Short("nonode@nohost".to_string()),
      start: Vec::new(),
      search_path: vec![]
    }
  }

  pub fn add_start(&mut self, data: &[&str]) {
    self
      .start
      .push(data.iter().map(|s| s.to_string()).collect())
  }

  /// Copy args from any string iterator source
  pub fn populate_with<ITER>(&mut self, mut iter: ITER)
  where
    ITER: Iterator<Item = String>,
  {
    loop {
      if let Some(s) = iter.next() {
        self.add_arg1(s.as_ref())
      } else {
        break;
      }
    }
  }

  /// Parses and adds one argument with no parameter
  pub fn add_arg1(&mut self, a1: &str) {
    self.parse_arg(&[a1]);
  }

  pub fn add_arg2(&mut self, a1: &str, a2: &str) {
    self.parse_arg(&[a1, a2]);
  }

  /// Parses and adds one argument with possible parameters which will be
  /// fetched from the mutable string-collection iterator
  pub fn parse_arg(&mut self, args: &[&str])
  {
    // assert at least one string is present in args
    let a = args[0];
    match a.as_ref() {
      "-sname" => {
        self.node = NodeName::Short(args[1].to_string());
      }
      "-name" => {
        self.node = NodeName::Full(args[1].to_string());
      }
      other => self.other_args.push(String::from(other)),
    }
  }

  /// Set node name without changing node mode
  pub fn set_node_name(&mut self, n: &str) {
    match self.node {
      NodeName::Full(_) => self.node = NodeName::Full(n.to_string()),
      NodeName::Short(_) => self.node = NodeName::Short(n.to_string()),
    }
  }
}
