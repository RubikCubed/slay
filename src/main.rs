use nix::sys::signal::{self, Signal};
use nix::unistd::Pid;
use std::env;
use std::process;

fn print_usage_and_exit() -> ! {
    eprintln!("Usage: slay <pid>");
    process::exit(1);
}

fn main() {
    let mut args = env::args();

    // Skip the first argument which is the program name
    args.next();

    if args.len() == 0 {
        print_usage_and_exit();
    }

    let signal = Signal::SIGKILL;

    for arg in args {
        match arg.parse::<i32>() {
            Ok(pid) => {
                let pid = Pid::from_raw(pid);
                if let Err(err) = signal::kill(pid, signal) {
                    eprintln!("Failed to send signal to {}: {}", pid, err);
                }
            }
            Err(_) => {
                eprintln!("Invalid PID: {}", arg);
            }
        }
    }
}
