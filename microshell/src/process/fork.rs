use nix::unistd::{execvp, fork, ForkResult};
use nix::sys::wait::*;

use super::signal;
use super::parse;

pub fn exec_cmd(_line: &str) {
    if !parse::judge_builtin_cmd(_line) {
        match unsafe { fork() } { 
            Ok(ForkResult::Parent { child, .. }) => {
                waitpid(child, None).unwrap();
            }   
            Ok(ForkResult::Child) => {
                signal::restore_signals();
                let cmd = parse::cmd_parse(_line);
                execvp(&cmd[0], &cmd).unwrap();
            }   
            Err(e) => {
                eprintln!("fork error: {}", e); 
            }
        }
    }
}
