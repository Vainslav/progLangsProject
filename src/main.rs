use termion::raw::IntoRawMode;
use std::io::{Write, stdout};
use termion::clear;

mod piece_table;
// use std::env;

fn main() {
    let hello = String::from("Hello ");
    let mut piece_table = piece_table::PieceTable::new();
    piece_table.assign_buffer(hello);
    let world = String::from("World!");
    piece_table.push( world);
    print!("{}\n", piece_table.get_text());
    piece_table.pop();
    print!("{}\n", piece_table.get_text());
    piece_table.remove(1, 5);
    print!("{}", piece_table.get_text());
}
