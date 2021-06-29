use std::env;
use std::process;
use std::ffi::*;
use nix::unistd::*;

static USAGE: &str = "Usage:\n  ruboxer [-h | --help | -v | --version]\n  ruboxer <directory> <command> [<arguments>]";
static VERSION: &str = "0.1.0";

fn main() {
  let args: Vec<String> = env::args().collect();
  let try_args_cstr: Vec<Result<CString, NulError>> = args.clone().into_iter().map(|x| CString::new(x)).collect();
  if try_args_cstr.iter().any(|x| match x { Ok(_) => false, _ => true }) {
    println!("Fatal error: interior NUL byte found in one or more arguments");
    process::exit(1);
  };
  let mut args_cstr: Vec<CString> = Vec::new();
  for x in try_args_cstr {
    match x {
      Ok(cstr) => { args_cstr.push(cstr); },
      _ => ()
    }
  }
  if args.len() < 2 {
    println!("{}", USAGE);
    process::exit(1);
  }
  if args.len() == 2 {
    match &args[1][..] {
      "-h" | "--help" => {
        println!("{}", USAGE);
        process::exit(0);
      },
      "-v" | "--version" => {
        println!("{}", VERSION);
        process::exit(0);
      },
      _ => {
        println!("{}", USAGE);
        process::exit(1);
      }
    }
  }
  match chdir(&args[1][..]) {
    Ok(()) => (),
    Err(error) => {
      println!("{}", error);
      process::exit(1);
    }
  };
  match chroot(&args[1][..]) {
    Ok(()) => (),
    Err(error) => {
      println!("{}", error);
      process::exit(1);
    }
  };
  match execvp(&args_cstr[2], &args_cstr[2..]) {
    Ok(_) => (),
    Err(error) => {
      println!("{}", error);
      process::exit(1);
    }
  };
}
