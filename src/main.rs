mod modes;
mod util;
mod managers;
mod event_lisner;
mod choose_file;

use event_lisner::get_event_reviever;
use managers::document_manager::Document;
use modes::mode_manager::ModeManager;

use std::env::args;

fn main(){
    let args: Vec<String> = args().collect();
    let mut file: String;
    if args.len() != 2{
        file = choose_file::choose_file();
    }else{
        file = args[1].clone();
    }

    let command_reciever = get_event_reviever();

    let mut document_manager = Document::new(file);

    let mut modes = ModeManager::new();
    modes.run(&mut document_manager, command_reciever);
}
 