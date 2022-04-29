mod process;

use nix::sys::wait::*;
use std::{ffi::CString, process::exit};
use nix::unistd::{execvp, fork, ForkResult};

use process::signal;
use process::builtin_cmd;
use process::display;

fn main() {
    signal::ignore_signals();
    while let Some(line) = display::read_line() {
        if !&line.is_empty() {
            match cmd_parse(&line) {
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

fn cmd_parse(_line: &str) -> Option<Vec<CString>> {
    let lines = _line.split(' ')
        .map(|s| s.to_string())
        .collect::<Vec<_>>();
    let mut cmd = Vec::new();

    for (i, argument) in lines.iter().enumerate() {
        if argument == "exit" {
            exit(0);
        }
        else if argument == "cd" {
            builtin_cmd::run_cd(lines[i + 1].clone());
            return None;
        }
        cmd.push(CString::new(argument.to_string()).unwrap());
    }
    Some(cmd)
}
