use termion::screen::AlternateScreen;
use termion::raw::RawTerminal;
use std::io::Stdout;

pub trait Mode{
    fn run(&mut self, stdout: &AlternateScreen<RawTerminal<Stdout>>);

    fn update(&self);
}