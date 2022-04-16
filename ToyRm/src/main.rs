use std::fs;
use std::env;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    fs::remove_file(filename)?;
    Ok(())
}
