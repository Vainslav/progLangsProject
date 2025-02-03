use termion::screen::{AlternateScreen, IntoAlternateScreen};
use termion::raw::{RawTerminal, IntoRawMode};

use std::collections::HashMap;
use std::io::{stdout, Stdout};

use crate::modes::mode_trait::Mode;
use crate::managers::document_manager::Document;

use super::insert_mode::InsertMode;

#[derive(PartialEq, Eq, Hash)]
enum Modes{
    Insert,
    Normal
}

static mut CURRENT_MODE: Modes = Modes::Insert;

pub struct ModeManager{
    modes: HashMap<Modes, Box<dyn Mode>>,
    screen: AlternateScreen<RawTerminal<Stdout>>,
}

impl ModeManager{
    pub fn new(document: Document) -> Self{
        let mut hash_map: HashMap<Modes, Box<dyn Mode>> = HashMap::new();
        hash_map.insert(Modes::Insert, Box::new(InsertMode::init(document)));
        ModeManager{
            modes: hash_map,
            screen: stdout().into_raw_mode().unwrap().into_alternate_screen().unwrap()
        }
    }

    pub fn run(&mut self){
        while true{
            if unsafe{CURRENT_MODE == Modes::Normal}{
                self.modes.get_mut(&Modes::Normal).unwrap().run(&self.screen);
            }
            else if unsafe{CURRENT_MODE == Modes::Insert}{
                self.modes.get_mut(&Modes::Insert).unwrap().run(&self.screen);
            }
        }
    }
}