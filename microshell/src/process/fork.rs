use nix::unistd::{execvp, fork, ForkResult};
use nix::sys::wait::*;
use std::ffi::CString;
use super::signal;

pub fn exec_cmd(cmd: Vec<CString>) {
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
