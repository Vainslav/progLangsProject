use termion::raw::IntoRawMode;
use std::io::{Write, stdout};
use termion::clear;
use termion::screen::IntoAlternateScreen;
use std::thread::{self, sleep};


mod piece_table;
mod text_manager;
use crate::text_manager::TextManager;


fn main() {
    let mut text_manager = TextManager::init("input_text");
    text_manager.run();
}
