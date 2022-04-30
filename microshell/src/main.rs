mod process;

use nix::sys::wait::*;
use nix::unistd::{execvp, fork, ForkResult};
 use std::ffi::CString;

use process::signal;
use process::display;
use process::parse;

fn main() {
    signal::ignore_signals();
    while let Some(line) = display::read_line() {
        if !&line.is_empty() {
            match parse::cmd_parse(&line) {
                Some(result) => { exec_cmd(result) }
                None => {}
            }
        }
    }
    println!();
}

fn exec_cmd(cmd: Vec<CString>) {
    match unsafe { fork() } {
        Ok(ForkResult::Parent { child, .. }) => {
            waitpid(child, None).unwrap();
        }
        Ok(ForkResult::Child) => {
            signal::restore_signals();

            execvp(&cmd[0], &cmd).unwrap();
        }
        Err(e) => {
            eprintln!("fork error: {}", e);
        }
    }
}
