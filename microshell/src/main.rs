mod process;

use process::signal;
use process::display;
use process::parse;
use process::fork;

fn main() {
    signal::ignore_signals();
    while let Some(line) = display::read_line() {
        if !&line.is_empty() {
            match parse::cmd_parse(&line) {
                Some(result) => { fork::exec_cmd(result) }
                None => {}
            }
        }
    }
    println!();
}
