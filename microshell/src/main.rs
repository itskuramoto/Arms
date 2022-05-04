mod process;

use process::signal;
use process::display;
use process::fork;

fn main() {
    signal::ignore_signals();
    while let Some(line) = display::read_line() {
        if !&line.is_empty() {
            fork::exec_cmd(&line);
        }
    }
    println!();
}
