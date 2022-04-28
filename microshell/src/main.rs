mod process;

use nix::sys::{
    wait::*,
};
use std::{
    ffi::CString,
    io::{stdin, stdout, Write},
    env,
    process::exit,
    path::Path,
};
use nix::unistd::{execvp, fork, ForkResult};
use colored::*;
use whoami;
use process::signal;

fn main() {
    signal::ignore_signals();
    while let Some(line) = read_line() {
        if !&line.is_empty() {
            match cmd_parse(&line) {
                Some(result) => { exec_cmd(result) }
                None => {}
            }
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
            run_cd(lines[i + 1].clone());
            return None;
        }
        cmd.push(CString::new(argument.to_string()).unwrap());
    }
    Some(cmd)
}

fn run_cd(_dir: String) {
    let path = Path::new(&_dir);
    match env::set_current_dir(path) {
        Ok(()) => {}
        Err(e) => { eprintln!("{}", e) }
    }
}
