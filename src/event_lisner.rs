use std::sync::mpsc::Receiver;

use termion::event::Event;
use termion::input::TermRead;

use std::io::stdin;
use std::sync::{mpsc, Mutex, MutexGuard, OnceLock};
use std::thread;
use std::time::Duration;

struct EventListener {
    event_reciever: Mutex<Receiver<Event>>,
}

impl EventListener {
    fn new() -> EventListener {
        let (tx, rx) = mpsc::channel::<Event>();

        thread::spawn(move || {
            let mut events = stdin().events();

            loop {
                let event = events
                    .next()
                    .map(|result| result.unwrap_or(Event::Unsupported(Vec::new())))
                    .unwrap_or(Event::Unsupported(Vec::new()));
                tx.send(event).unwrap();
                thread::sleep(Duration::from_millis(1));
            }
        });
        EventListener {
            event_reciever: Mutex::new(rx),
        }
    }

    pub fn get_reciever(&self) -> MutexGuard<Receiver<Event>> {
        self.event_reciever.lock().unwrap()
    }
}

static INSTANCE: OnceLock<EventListener> = OnceLock::new();

pub fn get_event_reviever() -> MutexGuard<'static, Receiver<Event>> {
    INSTANCE.get_or_init(|| EventListener::new()).get_reciever()
}
