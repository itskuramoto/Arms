use std::{env, path::Path};
use nix::fcntl::{open, OFlag};
use nix::sys::stat::Mode;
use nix::unistd::*;

pub fn run_cd(dir: &str) {
    let path = Path::new(dir);
    match env::set_current_dir(path) {
        Ok(()) => {}
        Err(e) => { eprintln!("{}", e) }
    }
}

pub fn redirect(path: &str) {
    let mode = Mode::S_IRWXU;
    let flag = OFlag::O_CREAT | OFlag::O_WRONLY | OFlag::O_TRUNC;
    match open_file(path, flag, mode) {
        Ok(()) => {}
        Err(e) => { eprintln!("{}", e) }
    }
}

pub fn over_redirect(path: &str) {
    let mode = Mode::S_IRWXU;
    let flag = OFlag::O_CREAT | OFlag::O_WRONLY | OFlag::O_APPEND;
    match open_file(path, flag, mode) {
        Ok(()) => {}
        Err(e) => { eprintln!("{}", e) }
    }
}


fn open_file(path: &str, flag: OFlag, mode: Mode) -> Result<(), String> {
    match open(path, flag, mode) {
      Ok(file) => match dup2(file, 1) {
        Ok(_) => match close(file) {
          Ok(_) => {}
          Err(_) => {
            return Err(format!("file clse error"));
          }
        },
        Err(_) => {
          return Err(format!("file dup2 error"));
        }
      },
      Err(_) => {
        return Err(format!("file open error"));
      }
    }
    return Ok(());
  }
