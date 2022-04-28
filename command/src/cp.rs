use std::io::{Error, Write, Read};
use std::fs::File;
use clap::Parser;

fn file_copy(_source: &String, _dest: &String) -> Result<(), Error> {
    let mut file = File::open(_source).expect("file not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    let mut f = File::create(_dest)?;
    f.write_all(contents.as_bytes())?;
    Ok(())
}

fn main() {
    let args = Args::parse();
    match file_copy(&args.source, &args.dest) {
        Ok(()) => {
        }
        Err(e) => {
            println!("Could not copy. Error:{}", e);
        }
    }
}

#[derive(Debug, Parser)]
#[clap(name = "struct", author, about, version)]

struct Args {
    /// source file
    source: String,

    /// dest file
    dest: String,
}
