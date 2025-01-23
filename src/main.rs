mod modes;
mod util;
mod managers;
mod terminal_state;

use managers::lines_manager::LinesManager;

use std::env::args;


fn main() -> Result<(), String>{
    let args: Vec<String> = args().collect();
    if args.len() != 2{
        Err("File not specified".to_string())?;
    }

    let mut lines_manager = LinesManager::init(&"".to_string());

    let mut text_manager = match InsertMode::init(&args[1]);
    text_manager.run();
    Ok(())
}
 