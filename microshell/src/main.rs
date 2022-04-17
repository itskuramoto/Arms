use nix::unistd::execvp;
use std::env;
use std::ffi::CString;

fn main() {
    let mut cmd = Vec::new();
    for argument in env::args().skip(1) {
        cmd.push(CString::new(argument.to_string()).unwrap());
    }
    execvp(&cmd[0], &cmd).unwrap();
}
