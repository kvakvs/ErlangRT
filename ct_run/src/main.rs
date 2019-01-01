use erlangrt::command_line_args::{DistMode, ErlStartArgs};
use std::env;

// const ERLNAME: &'static str = "erl";

enum CtMode {
  Normal,
  Vts,
  CtShell,
  Master,
  ErlShell,
}

fn main() {
  let in_args: Vec<String> = env::args().collect();
  // let emulator = get_default_emulator(&in_args[0]);
  // let emulator_str = emulator.to_str().unwrap();
  // env::set_var("ESCRIPT_NAME", emulator_str);
  let mut start_args = ErlStartArgs::new();

  // let mut cmd = Command::new(emulator);
  start_args.node_name = String::from("ct");
  let mut erl_args_pos: Option<usize> = None;
  start_args.dist_mode = DistMode::ShortName;
  let mut ct_mode = CtMode::Normal;
  let mut browser = String::new();

  let mut cnt = 1;
  while cnt < in_args.len() {
    if in_args[cnt] == "-erl_args" {
      erl_args_pos = Some(cnt);
    } else if in_args[cnt] == "-sname" {
      start_args.node_name = in_args[cnt + 1].clone();
      cnt += 1;
    } else if in_args[cnt] == "-name" {
      start_args.node_name = in_args[cnt + 1].clone();
      cnt += 1;
      start_args.dist_mode = DistMode::FullName;
    } else if erl_args_pos.is_none() {
      if in_args[cnt] == "-vts" {
        ct_mode = CtMode::Vts;
      } else if in_args[cnt] == "-browser" {
        browser = in_args[cnt + 1].clone();
        cnt += 1;
      } else if in_args[cnt] == "-shell" {
        ct_mode = CtMode::CtShell;
      } else if in_args[cnt] == "-ctmaster" {
        start_args.node_name = String::from("ct_master");
        ct_mode = CtMode::Master;
      } else if in_args[cnt] == "-ctname" {
        start_args.node_name = in_args[cnt + 1].clone();
        cnt += 1;
        ct_mode = CtMode::ErlShell;
      }
    }
    cnt += 1;
  }

  match ct_mode {
    CtMode::Vts => {
      if browser.is_empty() {
        start_args.add_start(&["ct_webtool", "script_start", "vts"]);
      } else {
        start_args.add_start(&["ct_webtool", "script_start", "vts", browser.as_str()]);
      }
      add_script_start(&mut start_args);
    }
    CtMode::CtShell => {
      add_script_start(&mut start_args);
    }
    CtMode::Normal => {
      add_script_start(&mut start_args);
      start_args.add_start(&["-s", "erlang", "halt"]);
    }
    CtMode::Master | CtMode::ErlShell => {}
  }

  // Run again through the args
  //  cnt = 1;
  // TODO: Parse the command line here, instead of copy-modifying
  //  while cnt < in_args.len() {
  //    if in_args[cnt] == "-erl_args" {
  //      cmd.arg("-ct_erl_args");
  //    } else if in_args[cnt] == "-sname" || in_args[cnt] == "-name" {
  //      cnt += 1;
  //    } else if erl_args_pos.is_some() && erl_args_pos.unwrap() > cnt {
  //      if in_args[cnt] == "-config" {
  //        cmd.arg("-ct_config");
  //      } else if in_args[cnt] == "-decrypt_key" {
  //        cmd.arg("-ct_decrypt_key");
  //      } else if in_args[cnt] == "-decrypt_file" {
  //        cmd.arg("-ct_decrypt_file");
  //      } else {
  //        cmd.arg(in_args[cnt].clone());
  //      }
  //    } else {
  //      cmd.arg(in_args[cnt].clone());
  //    }
  //    cnt += 1;
  //  }

  //  println!("{:?}", cmd);
  //  let mut child = cmd.spawn().unwrap();
  //  let exit_status = child.wait().unwrap();
  //  println!("erl exit status: {}", exit_status);
  erlangrt::lib_main::start_emulator(&start_args);
}

fn add_script_start(args: &mut ErlStartArgs) {
  args.add_start(&["-s", "ct_run", "script_start"]);
}

///// Trim program name from `args[0]` and append `erl`
// fn get_default_emulator(prog: &str) -> PathBuf {
//  return PathBuf::from("/home/kv/.asdf/shims/erl");
//
//  let p = PathBuf::from(prog);
//  match p.parent() {
//    None => {
//      return PathBuf::from(ERLNAME);
//    }
//    Some(parent) => {
//      return parent.join(Path::new(ERLNAME));
//    }
//  }
//}
