use whoami;
use colored::*;
use std::env;
use std::io::{stdin, stdout, Write};

pub fn read_line() -> Option<String> {
    display_prompt();

    let mut result = String::new();
    match stdin().read_line(&mut result) {
        Ok(size) => {
            if size == 0 {
                None
            } else {
                let result = result.trim_end();
                Some(result.to_string())
            }
        }
        Err(e) => {
            eprintln!("{}", e);
            None
        }
    }
}

fn display_prompt() {
    let current_path = env::current_dir().unwrap();

    match current_path.to_str() {
        None => { eprintln!("Path cannot be converted to string type"); },
        Some(strdir) => {
            let dir: Vec<&str> = strdir.split('/').collect();
            print!("[{}@{} {}]{}",
                whoami::username().yellow(),
                whoami::hostname(),
                dir[dir.len()-1],
                "$ ".red());
            stdout().flush().unwrap();
        },
    }
}

