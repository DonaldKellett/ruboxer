use std::env;
use std::process;
use std::ffi::*;
use nix::unistd::*;
use nix::sched::*;
use nix::sys::wait::*;
use nix::sys::stat::*;
use nix::fcntl::*;

static USAGE: &str = "Usage:\n  ruboxer [-h | --help | -v | --version]\n  ruboxer [--ns-pid <pid>] <directory> <command> [<arguments>]";
static VERSION: &str = "0.1.0";

fn main() {
  let args: Vec<String> = env::args().collect();
  let try_args_cstr: Vec<Result<CString, NulError>> = args.clone().into_iter().map(|x| CString::new(x)).collect();
  if try_args_cstr.iter().any(|x| match x { Ok(_) => false, _ => true }) {
    println!("Fatal error: interior NUL byte found in one or more arguments");
    process::exit(1);
  }
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
  let mut ns_pid: Option<i32> = None;
  let mut i = 1;
  while i < args.len() {
    match &args[i][..] {
      "--ns-pid" => {
        match ns_pid {
          Some(_) => {
            println!("Fatal error: --ns-pid option cannot be specified more than once");
            process::exit(1);
          },
          None => ()
        };
        i += 1;
        if i >= args.len() {
          println!("Fatal error: --ns-pid option should be followed by a PID number");
          process::exit(1);
        }
        let try_pid = args[i].parse::<i32>();
        match try_pid {
          Ok(pid) => {
            if pid < 1 {
              println!("Fatal error: the PID in --ns-pid must be a positive integer");
              process::exit(1);
            }
            ns_pid = Some(pid);
          },
          Err(error) => {
            println!("{}", error);
            process::exit(1);
          }
        };
      },
      _ => break
    }
    i += 1;
  }
  if i + 1 >= args.len() {
    println!("{}", USAGE);
    process::exit(1);
  }
  match ns_pid {
    Some(pid) => {
      let ns_pid_path = format!("/proc/{}/ns/pid", pid);
      match open(&ns_pid_path[..], OFlag::O_RDONLY, Mode::empty()) {
        Ok(pid_ns_fd) => match setns(pid_ns_fd, CloneFlags::CLONE_NEWPID) {
          Ok(()) => match close(pid_ns_fd) {
            Ok(()) => (),
            Err(error) => {
              println!("{}", error);
              process::exit(1);
            }
          },
          Err(error) => {
            println!("{}", error);
            process::exit(1);
          }
        },
        Err(error) => {
          println!("{}", error);
          process::exit(1);
        }
      };
    },
    None => {
      let mut unshare_flags = CloneFlags::empty();
      unshare_flags.insert(CloneFlags::CLONE_NEWPID);
      unshare_flags.insert(CloneFlags::CLONE_NEWNS);
      match unshare(unshare_flags) {
        Ok(()) => (),
        Err(error) => {
          println!("{}", error);
          process::exit(1);
        }
      };
    }
  };
  match unsafe{fork()} {
    Ok(ForkResult::Parent { .. }) => {
      match wait() {
        Ok(_) => (),
        Err(error) => {
          println!("{}", error);
          process::exit(1);
        }
      };
      process::exit(0);
    },
    Ok(ForkResult::Child) => (),
    Err(error) => {
      println!("{}", error);
      process::exit(1);
    }
  };
  match chdir(&args[i][..]) {
    Ok(()) => (),
    Err(error) => {
      println!("{}", error);
      process::exit(1);
    }
  };
  match chroot(&args[i][..]) {
    Ok(()) => (),
    Err(error) => {
      println!("{}", error);
      process::exit(1);
    }
  };
  match execvp(&args_cstr[i + 1], &args_cstr[(i + 1)..]) {
    Ok(_) => (),
    Err(error) => {
      println!("{}", error);
      process::exit(1);
    }
  };
}
