use clap::{App, Arg};
use std::fs::File;
use std::io::Read;

fn main() {
    let matches = App::new("MyApp")
        .version("0.1")
        .author("lion0024 <lion0024@github.com>")
        .about("Does awesome things")
        .arg(
            Arg::new("output")
                .index(1),
        )   
        .get_matches();
    
    if let Some(path) = matches.value_of("output") {
        let mut f = File::open(path).expect("file not found");

        let mut contents = String::new();
        f.read_to_string(&mut contents)
            .expect("something went wrong reading the file");

        println!("{}", contents);
    } else {
        println!("No file is specified");
    }   
}
