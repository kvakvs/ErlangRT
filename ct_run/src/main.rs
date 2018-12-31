use std::{
  env,
  path::{Path, PathBuf},
  process::Command,
};

const ERLNAME: &'static str = "erl";

enum DistMode {
  ShortName,
  FullName,
}

enum CtMode {
  Normal,
  Vts,
  CtShell,
  Master,
  ErlShell,
}

fn main() {
  let in_args: Vec<String> = env::args().collect();
  let emulator = get_default_emulator(&in_args[0]);
  let emulator_str = emulator.to_str().unwrap();
  env::set_var("ESCRIPT_NAME", emulator_str);

  let mut cmd = Command::new(emulator);
  let mut node_name = String::from("ct");
  let mut erl_args: Option<usize> = None;
  let mut dist_mode = DistMode::ShortName;
  let mut ct_mode = CtMode::Normal;
  let mut browser = String::new();

  let mut cnt = 1;
  while cnt < in_args.len() {
    if in_args[cnt] == "-erl_args" {
      erl_args = Some(cnt);
    } else if in_args[cnt] == "-sname" {
      node_name = in_args[cnt + 1].clone();
      cnt += 1;
    } else if in_args[cnt] == "-name" {
      node_name = in_args[cnt + 1].clone();
      cnt += 1;
      dist_mode = DistMode::FullName;
    } else if erl_args.is_none() {
      if in_args[cnt] == "-vts" {
        ct_mode = CtMode::Vts;
      } else if in_args[cnt] == "-browser" {
        browser = in_args[cnt + 1].clone();
        cnt += 1;
      } else if in_args[cnt] == "-shell" {
        ct_mode = CtMode::CtShell;
      } else if in_args[cnt] == "-ctmaster" {
        node_name = String::from("ct_master");
        ct_mode = CtMode::Master;
      } else if in_args[cnt] == "-ctname" {
        node_name = in_args[cnt + 1].clone();
        cnt += 1;
        ct_mode = CtMode::ErlShell;
      }
    }
    cnt += 1;
  }

  match dist_mode {
    DistMode::FullName => {
      cmd.args(&["-name", node_name.as_str()]);
    }
    DistMode::ShortName => {
      cmd.args(&["-sname", node_name.as_str()]);
    }
  }

  match ct_mode {
    CtMode::Vts => {
      cmd.args(&["-s", "ct_webtool", "script_start", "vts"]);
      if !browser.is_empty() {
        cmd.arg(browser);
      }
      cmd.args(&["-s", "ct_run", "script_start"]);
    }
    CtMode::CtShell => {
      cmd.args(&["-s", "ct_run", "script_start"]);
    }
    CtMode::Normal => {
      cmd.args(&["-s", "ct_run", "script_start"]);
      cmd.args(&["-s", "erlang", "halt"]);
    }
    CtMode::Master | CtMode::ErlShell => {}
  }

  // Run again through the args
  cnt = 1;
  while cnt < in_args.len() {
    if in_args[cnt] == "-erl_args" {
      cmd.arg("-ct_erl_args");
    } else if in_args[cnt] == "-sname" || in_args[cnt] == "-name" {
      cnt += 1;
    } else if erl_args.is_some() && erl_args.unwrap() > cnt {
      if in_args[cnt] == "-config" {
        cmd.arg("-ct_config");
      } else if in_args[cnt] == "-decrypt_key" {
        cmd.arg("-ct_decrypt_key");
      } else if in_args[cnt] == "-decrypt_file" {
        cmd.arg("-ct_decrypt_file");
      } else {
        cmd.arg(in_args[cnt].clone());
      }
    } else {
      cmd.arg(in_args[cnt].clone());
    }
    cnt += 1;
  }

  println!("{:?}", cmd);
  cmd.output().unwrap();
}

/// Trim program name from `args[0]` and append `erl`
fn get_default_emulator(prog: &str) -> PathBuf {
  let p = PathBuf::from(prog);
  match p.parent() {
    None => {
      return PathBuf::from(ERLNAME);
    }
    Some(parent) => {
      return parent.join(Path::new(ERLNAME));
    }
  }
}
