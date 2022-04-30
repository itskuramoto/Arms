use std::{env, path::Path};
use std::ffi::CString;

pub fn run_cd(_dir: String) {
    let path = Path::new(&_dir);
    match env::set_current_dir(path) {
        Ok(()) => {}
        Err(e) => { eprintln!("{}", e) }
    }
}

pub fn redirect(_cmd: Vec<CString>, _dir: String) {

}

pub fn over_redirect(_cmd: Vec<CString>, _dir: String) {
    let _path = Path::new(&_dir);
}
