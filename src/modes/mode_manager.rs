use termion::input::MouseTerminal;
use termion::screen::{AlternateScreen, IntoAlternateScreen};
use termion::raw::{RawTerminal, IntoRawMode};

use std::collections::HashMap;
use std::io::{stdout, Stdout};

use crate::modes::mode_trait::Mode;
use crate::managers::document_manager::Document;

use super::{insert_mode, normal_mode};

#[derive(PartialEq, Eq, Hash)]
enum Modes{
    Insert,
    Normal
}

static mut CURRENT_MODE: Modes = Modes::Insert;

pub struct ModeManager{
    modes: HashMap<Modes, fn(stdout: &mut MouseTerminal<RawTerminal<Stdout>>, document: &mut Document)>,
    screen: MouseTerminal<RawTerminal<Stdout>>,
}

impl ModeManager{
    pub fn new() -> Self{
        let mut hash_map: HashMap<Modes, fn(stdout: &mut MouseTerminal<RawTerminal<Stdout>>, document: &mut Document)> = HashMap::new();
        hash_map.insert(Modes::Insert, insert_mode::run);
        hash_map.insert(Modes::Normal, normal_mode::run);
        // hash_map.insert(Modes::Normal, Box::new(NormalMode::init(document)));
        ModeManager{
            modes: hash_map,
            screen: MouseTerminal::from(stdout().into_raw_mode().unwrap())
        }
    }

    pub fn run(&mut self, document: &mut Document){
        loop {
            if unsafe{CURRENT_MODE == Modes::Normal}{
                let func = self.modes.get_mut(&Modes::Normal).unwrap();
                func(&mut self.screen, document)
            }
            else if unsafe{CURRENT_MODE == Modes::Insert}{
                let func = self.modes.get_mut(&Modes::Insert).unwrap();
                func(&mut self.screen, document)
            }
            break
        }
    }
}