use termion::screen::AlternateScreen;
use termion::raw::RawTerminal;
use std::io::Stdout;

pub trait Mode{
    fn run(&mut self, stdout: &mut RawTerminal<Stdout>);

    fn update(&self, stdout: &mut RawTerminal<Stdout>);
}