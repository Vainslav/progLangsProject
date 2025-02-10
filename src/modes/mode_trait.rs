use termion::{input::MouseTerminal, raw::RawTerminal};
use std::io::Stdout;

pub trait Mode{
    fn run(&mut self, stdout: &mut MouseTerminal<RawTerminal<Stdout>>);

    fn update(&self, stdout: &mut MouseTerminal<RawTerminal<Stdout>>);
}