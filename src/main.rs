mod modes;
mod util;
mod managers;

use managers::document_manager::Document;
use modes::mode_manager::ModeManager;

use std::env::args;

fn main() -> Result<(), String>{
    let args: Vec<String> = args().collect();
    if args.len() != 2{
        Err("File not specified".to_string())?;
    }

    let document_manager = Document::new(args[1].clone());

    let mut modes = ModeManager::new(document_manager);
    modes.run();
    Ok(())
}
 