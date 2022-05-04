use std::{env, path::Path};

pub fn run_cd(dir: &str) {
    let path = Path::new(dir);
    match env::set_current_dir(path) {
        Ok(()) => {}
        Err(e) => { eprintln!("{}", e) }
    }
}
