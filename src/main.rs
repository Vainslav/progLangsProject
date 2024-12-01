use termion::raw::IntoRawMode;
use std::io::{Write, stdout};
use termion::clear;

mod piece_table;
// use std::env;

fn main() {
    // let args: Vec<String> = env::args().collect();
    // let file_path = &args[0];
    // dbg!(args);
    let mut stdout = stdout().into_raw_mode().unwrap();
    write!(stdout, "{}", clear::All).unwrap();
    write!(stdout, "Hey there.").unwrap();
}
