use std::env;
use std::process;
use std::ffi::*;
use nix::unistd::*;
use nix::sched::*;
use nix::sys::wait::*;
use nix::sys::stat::*;
use nix::fcntl::*;

static USAGE: &str = "Usage:\n  ruboxer [-h | --help | -v | --version]\n  ruboxer [--procns-pid <pid>] [--mem-max-bytes <bytes>] <directory> <command> [<arguments>]";
static VERSION: &str = "0.1.0";

static K: u64 = 1024;
static M: u64 = 1024 * K;
static G: u64 = 1024 * M;
static T: u64 = 1024 * G;
static P: u64 = 1024 * T;
static E: u64 = 1024 * P;

static MAX_E: u64 = 16;
static MAX_P: u64 = 1024 * MAX_E;
static MAX_T: u64 = 1024 * MAX_P;
static MAX_G: u64 = 1024 * MAX_T;
static MAX_M: u64 = 1024 * MAX_G;
static MAX_K: u64 = 1024 * MAX_M;

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
  let mut procns_pid: Option<i32> = None;
  let mut mem_max_bytes: Option<u64> = None;
  let mut i = 1;
  while i < args.len() {
    match &args[i][..] {
      "--procns-pid" => {
        match procns_pid {
          Some(_) => {
            println!("Fatal error: --procns-pid option cannot be specified more than once");
            process::exit(1);
          },
          None => ()
        };
        i += 1;
        if i >= args.len() {
          println!("Fatal error: --procns-pid option should be followed by a PID number");
          process::exit(1);
        }
        let try_pid = args[i].parse::<i32>();
        match try_pid {
          Ok(pid) => {
            if pid < 1 {
              println!("Fatal error: the PID in --procns-pid must be a positive integer");
              process::exit(1);
            }
            procns_pid = Some(pid);
          },
          Err(error) => {
            println!("{}", error);
            process::exit(1);
          }
        };
      },
      "--mem-max-bytes" => {
        match mem_max_bytes {
          Some(_) => {
            println!("Fatal error: --mem-max-bytes option cannot be specified more than once");
            process::exit(1);
          },
          None => ()
        };
        i += 1;
        if i >= args.len() {
          println!("Fatal error: --mem-max-bytes option must be followed by an unsigned integer");
          process::exit(1);
        }
        let try_bytes = args[i].parse::<u64>();
        match try_bytes {
          Ok(bytes) => { mem_max_bytes = Some(bytes); },
          Err(_) => {
            let args_i_len = args[i].len();
            if args_i_len == 0 {
              println!("Fatal error: byte count in --mem-max-bytes must not be empty");
              process::exit(1);
            }
            match &args[i][(args_i_len - 1)..args_i_len] {
              "K" => {
                let try_kbytes = args[i][..(args_i_len - 1)].parse::<u64>();
                match try_kbytes {
                  Ok(kbytes) => {
                    if kbytes >= MAX_K {
                      println!("Fatal error: maximum number of bytes ({}{} - 1) exceeded in --mem-max-bytes", MAX_K, "KiB");
                      process::exit(1);
                    }
                    mem_max_bytes = Some(kbytes * K);
                  },
                  Err(error) => {
                    println!("{}", error);
                    process::exit(1);
                  }
                };
              },
              "M" => {
                let try_mbytes = args[i][..(args_i_len - 1)].parse::<u64>();
                match try_mbytes {
                  Ok(mbytes) => {
                    if mbytes >= MAX_M {
                      println!("Fatal error: maximum number of bytes ({}{} - 1) exceeded in --mem-max-bytes", MAX_M, "MiB");
                      process::exit(1);
                    }
                    mem_max_bytes = Some(mbytes * M);
                  },
                  Err(error) => {
                    println!("{}", error);
                    process::exit(1);
                  }
                };
              },
              "G" => {
                let try_gbytes = args[i][..(args_i_len - 1)].parse::<u64>();
                match try_gbytes {
                  Ok(gbytes) => {
                    if gbytes >= MAX_G {
                      println!("Fatal error: maximum number of bytes ({}{} - 1) exceeded in --mem-max-bytes", MAX_G, "GiB");
                      process::exit(1);
                    }
                    mem_max_bytes = Some(gbytes * G);
                  },
                  Err(error) => {
                    println!("{}", error);
                    process::exit(1);
                  }
                };
              },
              "T" => {
                let try_tbytes = args[i][..(args_i_len - 1)].parse::<u64>();
                match try_tbytes {
                  Ok(tbytes) => {
                    if tbytes >= MAX_T {
                      println!("Fatal error: maximum number of bytes ({}{} - 1) exceeded in --mem-max-bytes", MAX_T, "TiB");
                      process::exit(1);
                    }
                    mem_max_bytes = Some(tbytes * T);
                  },
                  Err(error) => {
                    println!("{}", error);
                    process::exit(1);
                  }
                };
              },
              "P" => {
                let try_pbytes = args[i][..(args_i_len - 1)].parse::<u64>();
                match try_pbytes {
                  Ok(pbytes) => {
                    if pbytes >= MAX_P {
                      println!("Fatal error: maximum number of bytes ({}{} - 1) exceeded in --mem-max-bytes", MAX_P, "PiB");
                      process::exit(1);
                    }
                    mem_max_bytes = Some(pbytes * P);
                  },
                  Err(error) => {
                    println!("{}", error);
                    process::exit(1);
                  }
                };
              },
              "E" => {
                let try_ebytes = args[i][..(args_i_len - 1)].parse::<u64>();
                match try_ebytes {
                  Ok(ebytes) => {
                    if ebytes >= MAX_E {
                      println!("Fatal error: maximum number of bytes ({}{} - 1) exceeded in --mem-max-bytes", MAX_E, "EiB");
                      process::exit(1);
                    }
                    mem_max_bytes = Some(ebytes * E);
                  },
                  Err(error) => {
                    println!("{}", error);
                    process::exit(1);
                  }
                };
              },
              _ => {
                println!("Fatal error: invalid byte count in --mem-max-bytes");
                process::exit(1);
              }
            };
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
  match mem_max_bytes {
    Some(_) => (),
    None => { mem_max_bytes = Some(128 * M); }
  };
  match procns_pid {
    Some(pid) => {
      let procns_pid_path = format!("/proc/{}/ns/pid", pid);
      match open(&procns_pid_path[..], OFlag::O_RDONLY, Mode::empty()) {
        Ok(procns_pid_fd) => match setns(procns_pid_fd, CloneFlags::CLONE_NEWPID) {
          Ok(()) => match close(procns_pid_fd) {
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
  match mem_max_bytes {
    Some(bytes) => {
      let mem_cgroup_dir = format!("/sys/fs/cgroup/memory/{}", getpid());
      let mut mem_cgroup_dir_mode = Mode::empty();
      mem_cgroup_dir_mode.insert(Mode::S_IRWXU);
      mem_cgroup_dir_mode.insert(Mode::S_IRWXG);
      mem_cgroup_dir_mode.insert(Mode::S_IRWXO);
      match mkdir(&mem_cgroup_dir[..], mem_cgroup_dir_mode) { _ => () };
      let max_bytes_path = format!("{}/memory.limit_in_bytes", mem_cgroup_dir);
      match open(&max_bytes_path[..], OFlag::O_WRONLY, Mode::empty()) {
        Ok(max_bytes_fd) => {
          let bytes_bytes = format!("{}\n", bytes).into_bytes();
          match write(max_bytes_fd, &bytes_bytes[..]) {
            Ok(_) => (),
            Err(error) => {
              println!("{}", error);
              process::exit(1);
            }
          };
          match close(max_bytes_fd) {
            Ok(()) => (),
            Err(error) => {
              println!("{}", error);
              process::exit(1);
            }
          };
        },
        Err(error) => {
          println!("{}", error);
          process::exit(1);
        }
      };
      let swappiness_path = format!("{}/memory.swappiness", mem_cgroup_dir);
      match open(&swappiness_path[..], OFlag::O_WRONLY, Mode::empty()) {
        Ok(swappiness_fd) => {
          let zero_bytes = b"0\n";
          match write(swappiness_fd, &zero_bytes[..]) {
            Ok(_) => (),
            Err(error) => {
              println!("{}", error);
              process::exit(1);
            }
          };
          match close(swappiness_fd) {
            Ok(()) => (),
            Err(error) => {
              println!("{}", error);
              process::exit(1);
            }
          };
        },
        Err(error) => {
          println!("{}", error);
          process::exit(1);
        }
      };
      let tasks_path = format!("{}/tasks", mem_cgroup_dir);
      let mut tasks_oflag = OFlag::empty();
      tasks_oflag.insert(OFlag::O_WRONLY);
      tasks_oflag.insert(OFlag::O_APPEND);
      let tasks_mode = mem_cgroup_dir_mode;
      match open(&tasks_path[..], tasks_oflag, tasks_mode) {
        Ok(tasks_fd) => {
          let mypid_bytes = format!("{}\n", getpid()).into_bytes();
          match write(tasks_fd, &mypid_bytes[..]) {
            Ok(_) => (),
            Err(error) => {
              println!("{}", error);
              process::exit(1);
            }
          };
          match close(tasks_fd) {
            Ok(()) => (),
            Err(error) => {
              println!("{}", error);
              process::exit(1);
            }
          };
        },
        Err(error) => {
          println!("{}", error);
          process::exit(1);
        }
      };
    },
    None => panic!("Variable mem_max_bytes was unexpectedly None")
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
