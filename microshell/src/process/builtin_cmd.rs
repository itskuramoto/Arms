use std::{
    env,
    path::Path,
};

pub fn run_cd(_dir: String) {                                                     
    let path = Path::new(&_dir);
    match env::set_current_dir(path) {
        Ok(()) => {}
        Err(e) => { eprintln!("{}", e) }
    }
}
