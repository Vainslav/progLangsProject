use std::fs::read_to_string;
use std::io::stdin;
use std::io::{Write, stdout};

use termion::event::Key;
use termion::input::TermRead;
use termion::screen::IntoAlternateScreen;
use termion::raw::IntoRawMode;

use crate::piece_table::PieceTable;

struct CursorPos{
    x: usize,
    y: usize
}

pub struct TextManager{
    document: PieceTable,
    cursor_pos: CursorPos,
}


impl TextManager{
    pub fn init(file: &str) -> TextManager{
        let mut piece_table = PieceTable::new();
        let text_from_file = read_to_string(file).expect("File not found");
        piece_table.assign_buffer(text_from_file);
        TextManager{
            document: piece_table,
            cursor_pos: CursorPos{
                x: 1,
                y: 1
            }
        }
    }

    pub fn reload(&self){
        print!("{}{}", termion::clear::All, termion::cursor::Goto(1,1));
        //Терминал в raw mode не считывает каноничные лайн брейки нужно рестнуть курсор в начало строки
        print!("{}", self.document.get_text().replace("\n", "\n\r"));
        print!("{}", termion::cursor::Goto(self.cursor_pos.x as u16, self.cursor_pos.y as u16))
    }

    pub fn run(&mut self){
        let mut stdout = stdout().into_raw_mode().unwrap().into_alternate_screen().unwrap();
        self.reload();
        let stdin = stdin();
        for c in stdin.keys() {
            match c.unwrap() {
                Key::Ctrl('q') => {
                    break;
                }
                Key::Left => {
                    self.dec_x();
                    self.reload();
                }
                Key::Right => {
                    self.inc_x();
                    self.reload();
                }
                Key::Up => {
                    self.dec_y();
                    self.reload();
                }
                Key::Down => {
                    self.inc_y();
                    self.reload();
                }
                Key::Backspace => {
                    self.dec_x();
                    self.document.pop();
                    self.reload();
                }
                _=> {}
            }
            stdout.flush().unwrap();
        }
    }

    fn dec_x(&mut self){
        if self.cursor_pos.x == 1{
            return
        }
        self.cursor_pos.x -= 1
    }

    fn inc_x(&mut self){
        self.cursor_pos.x += 1
    }

    fn inc_y(&mut self){
        self.cursor_pos.y += 1
    }

    fn dec_y(&mut self){
        if self.cursor_pos.y == 1{
            return
        }
        self.cursor_pos.y -= 1
    }
}