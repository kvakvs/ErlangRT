use erlangrt::command_line_args::{ErlStartArgs};
use std::env;

// const ERLNAME: &'static str = "erl";

enum CtMode {
  Normal,
  Vts,
  CtShell,
  Master,
  ErlShell,
}

struct CtStartArgs {
  ct_mode: CtMode,
  browser: String,
}

impl CtStartArgs {
  pub fn new() -> Self {
    Self {
      ct_mode: CtMode::Normal,
      browser: String::new(),
    }
  }
}

fn main() {
  let in_args: Vec<String> = env::args().collect();
  let mut erl_args = ErlStartArgs::new(&in_args);
  erl_args.populate_with(in_args.iter());

  // Because we are running CT, set the default hostname to 'ct' shortname
  erl_args.add_arg2("-sname", "ct");

  let mut ct_args = CtStartArgs::new();

  // Take everything before erl_args option
  let before_erl_args: Vec<String> = in_args
    .iter()
    .take_while(|s| *s != "-erl_args")
    .map(|s| s.clone())
    .collect();
  // Take everything after erl_args skip the erl_args option itself
  let _after_erl_args: Vec<String> = in_args
    .iter()
    .skip_while(|s| *s != "-erl_args")
    .map(|s| s.clone())
    .skip(1)
    .collect();

  let mut b_iter = before_erl_args.iter();
  let _empty_s = String::new();
  loop {
    let a = match b_iter.next() {
      Some(s) => s,
      None => break
    };

    if a == "-vts" {
      ct_args.ct_mode = CtMode::Vts;
    } else if a == "-browser" {
      ct_args.browser = b_iter.next().unwrap().clone();
    } else if a == "-shell" {
      ct_args.ct_mode = CtMode::CtShell;
    } else if a == "-ctmaster" {
      // Note: possible bug here when -ct_master option overrides to short node
      // name without changing the node mode
      erl_args.set_node_name("ct_master");
      ct_args.ct_mode = CtMode::Master;
    } else if a == "-ctname" {
      // Note: possible bug here when -ctname option overrides to short node
      // name without changing the node mode
      erl_args.set_node_name(b_iter.next().unwrap());
      ct_args.ct_mode = CtMode::ErlShell;
    }
  }

  // Push ctmode args
  match ct_args.ct_mode {
    CtMode::Vts => {
      if ct_args.browser.is_empty() {
        erl_args.add_start(&["ct_webtool", "script_start", "vts"]);
      } else {
        erl_args.add_start(&[
          "ct_webtool",
          "script_start",
          "vts",
          ct_args.browser.as_str(),
        ]);
      }
      add_script_start(&mut erl_args);
    }
    CtMode::CtShell => {
      add_script_start(&mut erl_args);
    }
    CtMode::Normal => {
      add_script_start(&mut erl_args);
      erl_args.add_start(&["-s", "erlang", "halt"]);
    }
    CtMode::Master | CtMode::ErlShell => {}
  }

  // Push everything else

  let _modified_args: Vec<String> = in_args
    .iter()
    .map(|arg| {
      if arg == "-erl_args" {
        return "-ct_erl_args".to_string();
      }
      if arg == "-sname" || arg == "-name" {
        return arg.to_string();
      }
      arg.to_string()
//      if erl_args_pos.is_some() && erl_args_pos.unwrap() > cnt {
//        if in_args[cnt] == "-config" {
//          cmd.arg("-ct_config");
//        } else if in_args[cnt] == "-decrypt_key" {
//          cmd.arg("-ct_decrypt_key");
//        } else if in_args[cnt] == "-decrypt_file" {
//          cmd.arg("-ct_decrypt_file");
//        }
//      }
    })
    .collect();
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

  erl_args.search_path = vec![
    "priv/".to_string(),
    "../otp/erts/preloaded/ebin/".to_string(),
    "../otp/lib/stdlib/ebin/".to_string(),
  ];

  //  println!("{:?}", cmd);
  //  let mut child = cmd.spawn().unwrap();
  //  let exit_status = child.wait().unwrap();
  //  println!("erl exit status: {}", exit_status);
  erlangrt::lib_main::start_emulator(&mut erl_args);
  println!("ct_run: Finished.");
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
