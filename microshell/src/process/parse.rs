use std::{ffi::CString, process::exit};
use super::builtin_cmd;

pub fn cmd_parse(_line: &str) -> Option<Vec<CString>> {
    let lines = _line.split(' ')
        .map(|s| s.to_string())
        .collect::<Vec<_>>();
    let mut cmd = Vec::new();

    for argument in lines.iter() {
        if argument == "exit" {
            exit(0);
        }
        else if argument == "cd" {
            match lines.iter().next() {
                Some(args) => { builtin_cmd::run_cd(args.to_string()) }
                None => {}
            }
            return None;
        }
        cmd.push(CString::new(argument.to_string()).unwrap());
    }
    return Some(cmd);
}
