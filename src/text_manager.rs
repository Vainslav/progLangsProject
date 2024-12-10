use std::fs::read_to_string;
use std::io::stdin;
use std::io::{Write, stdout, Error};


use termion::event::Key;
use termion::input::TermRead;
use termion::screen::IntoAlternateScreen;
use std::fs;
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
    pub fn init(file: &str) -> Result<TextManager, Error>{
        let mut piece_table = PieceTable::new();
        let text_from_file = read_to_string(file)?;
        piece_table.assign_buffer(text_from_file);
        Ok(TextManager{
            document: piece_table,
            cursor_pos: CursorPos{
                x: 1,
                y: 1
            }
        })
    }

    pub fn reload(&self){
        print!("{}{}", termion::clear::All, termion::cursor::Goto(1,1));
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
                    let idx = self.get_document_index(&self.cursor_pos);
                    if self.cursor_pos.x == 1{
                        continue;
                    }
                    self.dec_x();
                    if idx > self.document.get_length(){}
                    else{
                        self.document.remove(self.get_document_index(&self.cursor_pos), 1);
                    }
                    self.reload();
                }
                Key::Delete => {
                    let idx = self.get_document_index(&self.cursor_pos);
                    if idx > self.document.get_length(){
                        continue;
                    }
                    self.document.remove(idx, 1);
                    self.reload();
                }
                Key::Char(ch)=> {
                    let idx = self.get_document_index(&self.cursor_pos);
                    if idx > self.document.get_length(){}
                    else{
                        self.document.insert(self.get_document_index(&self.cursor_pos), ch.to_string());
                        if ch == '\n'{
                            self.inc_y();
                            self.cursor_pos.x = 0;
                        }
                        else{
                            self.inc_x();
                        }
                    }
                    self.reload();
                }
                _ => {}
            }
            stdout.flush().unwrap();
        }
        fs::write("input_text", self.document.get_text()).expect("Unable to write file");
    }

    fn get_document_index(&self, cursor: &CursorPos) -> usize{
        let mut idx: usize = 0;
        for (i, line) in self.document.get_text().lines().enumerate(){
            if i == cursor.y - 1{
                break
            } 
            idx += line.chars().count() + 1;
        }
        return idx + cursor.x - 1
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
        if self.cursor_pos.y - 1 == self.document.get_num_lines(){
            return;
        } 
        self.cursor_pos.y += 1
    }

    fn dec_y(&mut self){
        if self.cursor_pos.y == 1{
            return
        }
        self.cursor_pos.y -= 1
    }
}