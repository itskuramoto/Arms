use std::env;
use std::io::{Error, Write};
use std::fs::File;
use std::io::Read;

fn file_copy(_fname: &String, _fcopy: &String) -> Result<(), Error> {
    let mut file = File::open(_fname).expect("file not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    let mut f = File::create(_fcopy)?;
    f.write_all(contents.as_bytes())?;
    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    match file_copy(&args[1], &args[2]) {
        Ok(()) => {
            // println!("OK:file_create ");
        }
        Err(e) => {
            println!("Could not copy. Error:{}", e);
        }
    }
}
