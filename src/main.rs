mod modes;
mod util;
mod managers;
mod event_lisner;
mod choose_file;

use event_lisner::get_event_reviever;
use managers::document_manager::Document;
use modes::mode_manager::ModeManager;

use std::env::args;

fn main() -> Result<(), String>{
    let args: Vec<String> = args().collect();
    if args.len() != 2{
        Err("File not specified".to_string())?;
    }

    let command_reciever = get_event_reviever();

    let mut document_manager = Document::new(args[1].clone());

    let mut modes = ModeManager::new();
    modes.run(&mut document_manager, command_reciever);

    Ok(())
}
 