use std::fs::File;
use std::io::{BufRead, BufReader};
use clap::Parser;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse(); 
    let mut cnt: u32 = 0;
    for result in BufReader::new(File::open(args.file)?).lines() {
        cnt += 1;
        let l = result?;
        if args.number
        {
            println!("{:>6}\t{}", cnt, l);
        }
        else
        {
            println!("{}", l);
        }
    }
    Ok(())
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

