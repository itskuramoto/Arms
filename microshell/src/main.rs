use nix::sys::wait::*;
use nix::sys::signal::*;
use nix::unistd::{execvp, fork, ForkResult};
use std::ffi::CString;
use std::io::{stdin, stdout, Write};

fn main() {
    ignore_signals();
    while let Some(line) = read_line() {
        if !&line.is_empty() {
            exec_cmd(&line);
        }
    }
}

fn read_line() -> Option<String> {
    print!("$ ");
    stdout().flush().unwrap();

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

fn exec_cmd(_line: &str) {
    match unsafe { fork() } {
        Ok(ForkResult::Parent { child, .. }) => {
            waitpid(child, None).unwrap();
        }
        Ok(ForkResult::Child) => {
            restore_signals();

            let lines = _line.split(' ')
                .map(|s| s.to_string())
                .collect::<Vec<_>>();
            let mut cmd = Vec::new();

            for argument in lines {
                cmd.push(CString::new(argument.to_string()).unwrap());
            }
            execvp(&cmd[0], &cmd).unwrap();
        }
        Err(e) => {
            eprintln!("fork error: {}", e);
        }
    }
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
