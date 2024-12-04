mod piece_table;
mod text_manager;
use crate::text_manager::TextManager;
use std::env::args;


fn main() {
    let args: Vec<String> = args().collect();
    if args.len() != 2{
        panic!("bad args");
    }
    let mut text_manager = TextManager::init(&args[1]);
    text_manager.run();
}
