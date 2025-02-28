use crate::managers::document_manager::Document;

use std::sync::MutexGuard;
use std::thread;
use std::sync::mpsc;
use std::time::Duration;

use termion::raw::RawTerminal;
use termion::input::MouseTerminal;
use termion::screen::AlternateScreen;
use termion::terminal_size;
use termion::event::Event;
use termion::input::TermRead;

use std::io::Stdout;
use std::io::stdin;       

use std::sync::mpsc::Receiver;

fn update(stdout: &mut AlternateScreen<MouseTerminal<RawTerminal<Stdout>>>, document: &mut Document) {
    
}

pub fn run(stdout: &mut AlternateScreen<MouseTerminal<RawTerminal<Stdout>>>, document: &mut Document, command_receiever: MutexGuard<'static, Receiver<Event>,>){
    let mut window_size = terminal_size().unwrap();

    let (tx, rx) = mpsc::channel::<Event>();

    thread::spawn(move || {
        let mut events = stdin().events();

        loop{
            let event = events.next()
            .map(|result| result.unwrap_or(Event::Unsupported(Vec::new())))
            .unwrap_or(Event::Unsupported(Vec::new()));
            tx.send(event).unwrap();
            thread::sleep(Duration::from_millis(1));
        }
    });

    loop{
        if window_size != terminal_size().unwrap(){
            window_size = terminal_size().unwrap();
            update(stdout, document);
        }

        match rx.try_recv(){
            Ok(event) => {}
            Err(_) => {}
        }
    }
}