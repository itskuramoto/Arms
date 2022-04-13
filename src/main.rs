use std::env;

fn main() {
    let mut str = "".to_string();
    for argument in env::args().skip(1) {
        str.push_str(&argument);
        str.push_str(" ");
    }
    println!("{}", str.trim());
}
