use std::{ffi::CString, process::exit};
use super::builtin_cmd;

pub fn cmd_parse(_line: &str) -> Vec<CString> {
    let lines = _line.split(' ')
        .map(|s| s.to_string())
        .collect::<Vec<_>>();
    let mut cmd = Vec::new();
    let mut cnt = 0;
    loop {
        if lines.len() <= cnt {
            break;
        }
        match &*lines[cnt] { 
            ">>"    => { cnt += 1; builtin_cmd::over_redirect(&lines[cnt]) }
            ">"     => { cnt += 1; builtin_cmd::redirect(&lines[cnt]) }
            _       => { cmd.push(CString::new(lines[cnt].to_string()).unwrap()) }
        }
        cnt += 1;
    }
    cmd
}

pub fn judge_builtin_cmd(_line: &str) -> bool {
    let lines = _line.split(' ')
        .map(|s| s.to_string())
        .collect::<Vec<_>>();
    match &*lines[0] { 
        "exit"  => { exit(0) }
        "cd"    => { builtin_cmd::run_cd(&lines[1]) }
        _       => { return false } 
    }
    return true;
}

