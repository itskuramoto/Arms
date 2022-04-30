use std::{ffi::CString, process::exit};
use super::builtin_cmd;

pub fn cmd_parse(_line: &str) -> Option<Vec<CString>> {
    let lines = _line.split(' ')
        .map(|s| s.to_string())
        .collect::<Vec<_>>();
    let mut cmd = Vec::new();
    match get_iter_next(lines.clone()) {
        Some(iter_next) => {
            for argument in lines.iter() {
                if check_builtin_cmd(argument, iter_next.clone(), cmd.clone()) {
                    return None;
                }
                else
                {
                    cmd.push(CString::new(argument.to_string()).unwrap());
                }
            }
            return Some(cmd);
        }
        None => { return None; }
    }
}

fn get_iter_next(lines: Vec<String>) -> Option<String> {
    let iter_next;
    match lines.iter().next() {
        Some(args) => { iter_next = args }
        None => { return None; }
    }
    Some(iter_next.to_string())
}

fn check_builtin_cmd(argument: &str, iter_next: String, cmd: Vec<CString>) -> bool {
    let mut is_ok = true;
    match argument {
        "exit"  => { exit(0) } 
        "cd"    => { builtin_cmd::run_cd(iter_next) }
        ">>"    => { builtin_cmd::over_redirect(cmd, iter_next) }
        ">"     => { builtin_cmd::redirect(cmd, iter_next) }
        _       => { is_ok = false }
    }
    is_ok
} 
