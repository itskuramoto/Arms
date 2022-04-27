use nix::sys::{
    wait::*,
    signal::*,
};
use std::{
    ffi::CString,
    io::{stdin, stdout, Write},
    env,
};
use nix::unistd::{execvp, fork, ForkResult};
use colored::*;
use whoami;

fn main() {
    ignore_signals();
    while let Some(line) = read_line() {
        if !&line.is_empty() {
            exec_cmd(cmd_parse(&line));
        }
    }
}

fn display_prompt() {
    let current_path = env::current_dir().unwrap();

    match current_path.to_str() {
        None => { eprintln!("Path cannot be converted to string type"); },
        Some(strdir) => {
            let dir: Vec<&str> = strdir.split('/').collect();
            print!("[{}@{} {}]{}",
                whoami::username().yellow(),
                whoami::hostname(),
                dir[dir.len()-1],
                "$ ".red());
            stdout().flush().unwrap();
        },
    }
}


fn read_line() -> Option<String> {
    display_prompt();

    let mut result = String::new();
    match stdin().read_line(&mut result) {
        Ok(size) => {
            if size == 0 {
                None
            } else {
                let result = result.trim_end();
                Some(result.to_string())
            }
        }
        Err(e) => {
            eprintln!("{}", e);
            None
        }
    }
}

fn exec_cmd(cmd: Vec<CString>) {
    match unsafe { fork() } {
        Ok(ForkResult::Parent { child, .. }) => {
            waitpid(child, None).unwrap();
        }
        Ok(ForkResult::Child) => {
            restore_signals();

            execvp(&cmd[0], &cmd).unwrap();
        }
        Err(e) => {
            eprintln!("fork error: {}", e);
        }
    }
}

fn cmd_parse(_line: &str) -> Vec<CString> {
    let lines = _line.split(' ')
        .map(|s| s.to_string())
        .collect::<Vec<_>>();
    let mut cmd = Vec::new();

    for argument in lines {
        cmd.push(CString::new(argument.to_string()).unwrap());
    }
    cmd
}


fn ignore_signals() {
    let sa = SigAction::new(SigHandler::SigIgn, SaFlags::empty(), SigSet::empty());
    unsafe {
        sigaction(Signal::SIGINT, &sa).unwrap();
        sigaction(Signal::SIGQUIT, &sa).unwrap();
    }
}

fn restore_signals() {
    let sa = SigAction::new(SigHandler::SigDfl, SaFlags::empty(), SigSet::empty());
    unsafe {
        sigaction(Signal::SIGINT, &sa).unwrap();
        sigaction(Signal::SIGQUIT, &sa).unwrap();
    }
}
