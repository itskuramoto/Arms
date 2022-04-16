use std::fs;
use clap::Parser;

fn main() -> std::io::Result<()> {
    let args = Args::parse();
    let metadata = fs::metadata(&args.file)?;
    if metadata.is_file() {
        fs::remove_file(&args.file)?;
    } else {
        fs::remove_dir(&args.file)?;
    }
    Ok(())
}

#[derive(Debug, Parser)]
struct Args {
    /// file path
    file: String,
}
