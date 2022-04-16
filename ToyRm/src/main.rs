use std::fs;
use clap::Parser;
use std::io::Error;

fn main() {
    let args = Args::parse();
    match file_delete(&args.file) {
        Ok(()) => {
        }
        Err(e) => {
            println!("Error:{}", e);
        }
    }
}

fn file_delete(_file: &String) -> Result<(), Error> {
    let metadata = fs::metadata(_file)?;
    if metadata.is_file() {
        fs::remove_file(_file).expect("File could not be deleted.");
    } else {
        fs::remove_dir(_file).expect("Directory could not be deleted.");
    }
    Ok(())
}

#[derive(Debug, Parser)]
struct Args {
    /// file path
    file: String,
}
