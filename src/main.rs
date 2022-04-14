use std::fs::File;
use std::io::Read;
use clap::Parser;

fn main() {
    let args = Args::parse(); 
    let mut f = File::open(args.file).expect("file not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    println!("{}", contents);
}

#[derive(Debug, Parser)]
#[clap(name = "struct", author, about, version)]

struct Args {
    /// file path
    file: String,
}

