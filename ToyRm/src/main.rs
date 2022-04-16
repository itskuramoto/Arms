use std::fs;
use clap::Parser;

fn main() -> std::io::Result<()> {
    let args = Args::parse();
    let filename = &args.file;
    fs::remove_file(filename)?;
    Ok(())
}

#[derive(Debug, Parser)]
struct Args {
    /// file path
    file: String,
}
