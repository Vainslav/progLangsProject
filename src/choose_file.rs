use termion::{input::MouseTerminal, raw::IntoRawMode, screen::{AlternateScreen, IntoAlternateScreen}, terminal_size};

use std::io::stdout;

use crate::event_lisner::get_event_reviever;

fn update(stdout: AlternateScreen<MouseTerminal<termion::raw::RawTerminal<std::io::Stdout>>>){
    
}

pub fn choose_file(){
    let stdout = MouseTerminal::from(stdout().into_raw_mode().unwrap()).into_alternate_screen().unwrap();
    let command_listener = get_event_reviever();

    let mut window_size = terminal_size().unwrap();

    loop {

    }
}