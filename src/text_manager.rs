use std::collections::VecDeque;
use std::fs::read_to_string;
use std::io::stdin;
use std::io::{Write, stdout, Error};
use std::cmp::{min, max};


use termion::event::Key;
use termion::input::TermRead;
use termion::screen::IntoAlternateScreen;
use std::fs;
use termion::raw::IntoRawMode;

use crate::piece_table::PieceTable;
use crate::lines_handler::LinesHandler;

struct CursorPos{
    x: usize,
    y: usize
}

pub struct TextManager{
    document: PieceTable,
    cursor_pos: CursorPos,
    lines_handler: LinesHandler,
    undo_redo: VecDeque<PieceTable>,
}


impl TextManager{
    pub fn init(file: &str) -> Result<TextManager, Error>{
        let mut piece_table = PieceTable::new();
        let text_from_file = read_to_string(file)?;
        let lines_handler = LinesHandler::init(&text_from_file);
        piece_table.assign_buffer(text_from_file);
        Ok(TextManager{
            document: piece_table,
            cursor_pos: CursorPos{
                x: 1,
                y: 1
            },
            lines_handler: lines_handler,
            undo_redo: VecDeque::new(),
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
                Key::Ctrl('z') => {
                    let new_document = self.undo_redo_pop();
                    if !new_document.is_none(){
                        self.document = new_document.unwrap();
                        self.update_lines_lenghts();
                        self.reload();
                    }
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
                    self.cursor_pos.x = min(self.cursor_pos.x, self.get_line_length(self.cursor_pos.y - 1) + 1);
                    self.reload();
                }
                Key::Down => {
                    self.inc_y();
                    self.cursor_pos.x = min(self.cursor_pos.x, self.get_line_length(self.cursor_pos.y - 1) + 1);
                    self.reload();
                }
                Key::Backspace => {
                    let idx = self.get_document_index(&self.cursor_pos);
                    if self.cursor_pos.x == 0{
                        continue;
                    }
                    self.dec_x();
                    self.undo_redo_push(self.document.clone());
                    if self.cursor_pos.x == 1 {
                        self.document.remove(self.get_document_index(&self.cursor_pos) - 1, 1);
                    }
                    else{
                        self.document.remove(self.get_document_index(&self.cursor_pos), 1);
                    }
                    self.update_lines_lenghts();
                    self.reload();
                }
                Key::Delete => {
                    let idx = self.get_document_index(&self.cursor_pos);
                    if idx > self.document.get_length(){
                        continue;
                    }
                    self.undo_redo_push(self.document.clone());
                    self.document.remove(idx, 1);
                    self.update_lines_lenghts();
                    self.reload();
                }
                Key::Char(ch)=> {
                    let idx = self.get_document_index(&self.cursor_pos);
                    if idx > self.document.get_length(){}
                    else{
                        self.undo_redo_push(self.document.clone());
                        self.document.insert(self.get_document_index(&self.cursor_pos), ch.to_string());
                        if ch == '\n'{
                            self.update_lines_lenghts();
                            self.inc_y();
                            self.cursor_pos.x = 1;
                        }
                        else{
                            self.increment_lenght(self.cursor_pos.y - 1);
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
        idx + cursor.x - 1
    }

    fn dec_x(&mut self){
        if self.cursor_pos.x == 1 && self.cursor_pos.y != 1{
            self.dec_y();
            self.cursor_pos.x = self.get_line_length(self.cursor_pos.y - 1) + 1;
            return
        }
        self.cursor_pos.x = max(self.cursor_pos.x - 1, 1); 
    }

    fn inc_x(&mut self){
        if self.cursor_pos.x == self.get_line_length(self.cursor_pos.y - 1) + 1 && self.cursor_pos.y != self.get_num_lines(){
            self.inc_y();
            self.cursor_pos.x = 1;
        }else{
            self.cursor_pos.x = min(self.cursor_pos.x + 1, self.get_line_length(self.cursor_pos.y - 1) + 1)
        }
    }

    fn inc_y(&mut self){
        if self.cursor_pos.y == self.get_num_lines(){
            return;
        } 
        self.cursor_pos.y += 1;
    }

    fn dec_y(&mut self){
        if self.cursor_pos.y == 1{
            return
        }
        self.cursor_pos.y -= 1;
    }

    fn update_lines_lenghts(&mut self){
        self.lines_handler.recalculate_lenghts(self.document.get_text());
    }

    fn increment_lenght(&mut self, line: usize){
        self.lines_handler.increment_lenght(line);
    }

    fn get_line_length(&self, line: usize) -> usize{
        self.lines_handler.get_line_lenght(line)
    }

    fn get_num_lines(&self) -> usize{
        self.lines_handler.get_num_lines()
    }

    fn undo_redo_push(&mut self, document: PieceTable){
        if self.undo_redo.len() == 10{
            self.undo_redo.pop_back();
        }
        self.undo_redo.push_front(document);
    } 

    fn undo_redo_pop(&mut self) -> Option<PieceTable>{
        self.undo_redo.pop_front()
    }
}