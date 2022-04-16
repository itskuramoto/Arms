use std::env;

fn main() {
    let mut str = "".to_string();
    let mut flg: bool = false;

    for argument in env::args().skip(1) {
        if argument == "-n" {
            flg = true;
            continue;
        }
        str.push_str(&argument);
        str.push_str(" ");
    }

    if flg {
        print!("{}", str.trim());
    } else {
        println!("{}", str.trim());
    }
}
