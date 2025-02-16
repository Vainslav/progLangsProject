use crate::managers::document_manager::Document;

use super::mode_trait::Mode;

use termion::raw::RawTerminal;
use termion::input::MouseTerminal;
use termion::screen::AlternateScreen;

use std::io::Stdout;        

pub fn run(stdout: &mut AlternateScreen<MouseTerminal<RawTerminal<Stdout>>>, document: &mut Document){

}

fn update(stdout: &mut AlternateScreen<MouseTerminal<RawTerminal<Stdout>>>, document: &mut Document) {
    
}