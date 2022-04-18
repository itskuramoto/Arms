use nix::sys::wait::*;
use nix::unistd::{execvp, fork, ForkResult};
use std::ffi::CString;
use std::io::{stdin, stdout, Write};

fn main() {
    loop {
        let line = read_line();
        if !&line.is_empty() {
            exec_cmd(&line);
        }
    }
}

fn read_line() -> String {
    print!("$ ");
    stdout().flush().unwrap();

    let mut result = String::new();
    match stdin().read_line(&mut result) {
        Ok(size) => {
            if size == 0 {
                return "".to_string();
            } else {
                let result = result.trim_end();
                return result.to_string();
            }
        }
        Err(e) => {
            eprintln!("{}", e);
            return "".to_string();
        }
    }
}

fn exec_cmd(_line: &str) {
    match unsafe { fork() } {
        Ok(ForkResult::Parent { child, .. }) => {
            waitpid(child, None).unwrap();
        }
        Ok(ForkResult::Child) => {
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
