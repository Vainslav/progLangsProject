use termion::screen::{AlternateScreen, IntoAlternateScreen};
use termion::raw::{RawTerminal, IntoRawMode};
use std::io::{Stdin, Stdout, stdin, stdout};
use crate::managers::cursor_manager::CursorPos;


enum Modes{
    Insert,
    Normal
}

pub struct TerminalState{
    cursor_pos: CursorPos,
    screen: AlternateScreen<RawTerminal<Stdout>>,
    current_mode: Modes
}

impl TerminalState{
    pub fn new() -> TerminalState{
        TerminalState{
            cursor_pos: CursorPos::new(1,1),
            screen: stdout().into_raw_mode().unwrap().into_alternate_screen().unwrap(),
            current_mode: Modes::Normal,
        }
    }

    pub fn get_cursor_pos(&self) -> &CursorPos{
        &self.cursor_pos
    }

    pub fn get_mut_cursor_pos(&mut self) ->&mut CursorPos{
        &mut self.cursor_pos
    }

    pub fn get_screen(&self) -> &AlternateScreen<RawTerminal<Stdout>>{
        &self.screen
    }

    pub fn get_current_mode(&self) -> &Modes{
        &self.current_mode
    }
}