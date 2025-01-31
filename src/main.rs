mod piece_table;
mod text_manager;
mod lines_handler;
mod undo_redo;
use crate::text_manager::TextManager;
use std::env::args;
use std::io::ErrorKind::NotFound;


fn main() -> Result<(), String>{
    let args: Vec<String> = args().collect();
    if args.len() != 2{
        Err("File not specified".to_string())?;
    }
    let mut text_manager = match TextManager::init(&args[1]){
        Ok(text_manager) => text_manager,
        Err(error) => 
        if error.kind() == NotFound{
            Err("File not found".to_string())?
        }else{
            Err("Unhandled errror".to_string())?
        }

    };
    text_manager.run();
    Ok(())
}
 