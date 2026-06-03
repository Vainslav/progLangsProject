use termion::event::Event;
use termion::input::MouseTerminal;
use termion::raw::{IntoRawMode, RawTerminal};
use termion::screen::{AlternateScreen, IntoAlternateScreen};

use std::collections::HashMap;
use std::io::{stdout, Stdout};
use std::sync::mpsc::Receiver;
use std::sync::MutexGuard;

use crate::managers::document_manager::Document;

use super::insert_mode;

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
enum Modes {
    Insert,
}

static mut CURRENT_MODE: Modes = Modes::Insert;

pub struct ModeManager {
    mode_handlers: HashMap<
        Modes,
        fn(
            stdout: &mut AlternateScreen<MouseTerminal<RawTerminal<Stdout>>>,
            document: &mut Document,
            command_receiever: MutexGuard<'static, Receiver<Event>>,
        ),
    >,
    screen: AlternateScreen<MouseTerminal<RawTerminal<Stdout>>>,
}

impl ModeManager {
    pub fn new() -> Self {
        let mut hash_map: HashMap<
            Modes,
            fn(
                stdout: &mut AlternateScreen<MouseTerminal<RawTerminal<Stdout>>>,
                document: &mut Document,
                command_receiever: MutexGuard<'static, Receiver<Event>>,
            ),
        > = HashMap::new();
        hash_map.insert(Modes::Insert, insert_mode::run);
        ModeManager {
            mode_handlers: hash_map,
            screen: MouseTerminal::from(stdout().into_raw_mode().unwrap())
                .into_alternate_screen()
                .unwrap(),
        }
    }

    pub fn run(
        &mut self,
        document: &mut Document,
        command_receiever: MutexGuard<'static, Receiver<Event>>,
    ) {
        loop {
            match unsafe { CURRENT_MODE } {
                Modes::Insert => {
                    let func = self.mode_handlers.get_mut(&Modes::Insert).unwrap();
                    func(&mut self.screen, document, command_receiever)
                }
                _ => {}
            }
            break;
        }
    }
}
