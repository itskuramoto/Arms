use clap::Parser;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn cat() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let file = File::open(args.file)?;
    for (count, result) in BufReader::new(file).lines().enumerate() {
        if args.number {
            println!("{:>6}\t{}", count + 1, result?);
        } else {
            println!("{}", result?)
        }
    }
    Ok(())
}

fn main() {
    match cat() {
        Ok(()) => {}
        Err(e) => {
            println!("{}", e)
        }
    }
}

#[derive(Debug, Parser)]
#[clap(name = "struct", author, about, version)]

struct Args {
    /// file path
    file: String,

    /// Line numbering on all lines
    #[clap(short, long)]
    number: bool,
}
