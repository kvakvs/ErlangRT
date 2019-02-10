use crate::{
  emulator::heap::Heap,
  fail::RtResult,
  term::{
    lterm::LTerm,
    term_builder::{list_builder::build_erlstr_from_utf8, ListBuilder},
  },
};

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
  /// Whole command line, before parse
  command_line: Vec<String>,
  /// Storage for other unknown args
  other_args: Vec<String>,
  pub node: NodeName,
  /// Which modules:functions to start (option -s m f arg1,...)
  pub start: Vec<Vec<String>>,
  pub search_path: Vec<String>,

  /// Small heap only for storing command line available globally
  arg_heap: Heap,
  /// Command line is stored here when built, otherwise is a non value
  args_term: LTerm,
}

impl ErlStartArgs {
  pub fn new(raw_command_line: &Vec<String>) -> Self {
    Self {
      command_line: raw_command_line.clone(),
      other_args: Vec::new(),
      node: NodeName::Short("nonode@nohost".to_string()),
      start: Vec::new(),
      search_path: vec![],
      arg_heap: Heap::new(1024),
      args_term: LTerm::non_value(),
    }
  }

  pub fn add_start(&mut self, data: &[&str]) {
    self
      .start
      .push(data.iter().map(|s| s.to_string()).collect())
  }

  /// Copy args from any string iterator source
  pub fn populate_with<'a, ITER>(&'a mut self, mut iter: ITER)
  where
    ITER: Iterator<Item = &'a String>,
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
  pub fn parse_arg(&mut self, args: &[&str]) {
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

  /// Using the local small heap build list of strings on it; Then return the
  /// list value, which also is cached in the args struct.
  pub fn get_command_line_list(&mut self) -> RtResult<LTerm> {
    if self.args_term.is_value() {
      return Ok(self.args_term);
    }

    let mut lb = unsafe { ListBuilder::new(&mut self.arg_heap) }?;
    for a in self.command_line.iter() {
      unsafe {
        let erl_str = build_erlstr_from_utf8(a.as_str(), &mut self.arg_heap)?;
        lb.append(erl_str);
      }
    }
    self.args_term = lb.make_term();
    Ok(self.args_term)
  }
}
