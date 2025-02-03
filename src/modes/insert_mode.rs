use std::io::stdin;
use std::cmp::{min, max};

use termion::event::Key;
use termion::input::TermRead;
use termion::screen::AlternateScreen;
use termion::raw::RawTerminal;

use std::io::Stdout;
use std::io::Write;

use crate::managers::document_manager::Document;
use crate::util::reversable_function::ReversableFunction;
use crate::util::reversable_function::Funcs;
use crate::modes::mode_trait::Mode;
use crate::managers::cursor_manager::CursorPos;

pub struct InsertMode{
    document: Document,
}

impl Mode for InsertMode{
    fn update(&self){
        print!("{}{}", termion::clear::All, termion::cursor::Goto(1,1));
        print!("{}", self.document.get_text().replace("\n", "\n\r"));
        print!("{}", termion::cursor::Goto(self.document.get_cursor().get_x() as u16, self.document.get_cursor().get_y() as u16))
    }

    fn run(&mut self, stdout: &AlternateScreen<RawTerminal<Stdout>>){
        self.update();
        let stdin = stdin();
        for c in stdin.keys() {
            match c.unwrap() {
                Key::Ctrl('q') => {
                    self.document.save();
                    break;
                }
                Key::Ctrl('c') => {
                    break;
                }
                Key::Ctrl('z') => {
                    self.document.undo();
                    self.update();
                }
                Key::Ctrl('y') => {
                    self.document.redo();
                    self.update();
                }
                Key::Left => {
                    self.document.get_cursor_mut().dec_x();
                    self.update();
                }
                Key::Right => {
                    self.document.get_cursor_mut().inc_x();
                    self.update();
                }
                Key::Up => {
                    self.document.get_cursor_mut().dec_y();
                    // self.cursor_pos.x = min(self.cursor_pos.x, self.get_line_length(self.cursor_pos.y - 1) + 1);
                    self.update();
                }
                Key::Down => {
                    self.document.get_cursor_mut().inc_y();
                    // self.cursor_pos.x = min(self.cursor_pos.x, self.get_line_length(self.cursor_pos.y - 1) + 1);
                    self.update();
                }
                Key::Backspace => {
                    let idx = self.get_document_index(self.document.get_cursor());
                    if self.document.get_cursor().get_x() == 1 && self.document.get_cursor().get_y() == 1{
                        continue;
                    }
                    self.document.get_cursor_mut().dec_x();
                    if self.document.get_cursor().get_x() == 1 {
                        let str = self.document.remove(self.get_document_index(self.document.get_cursor()), 1);
                        self.document.push_to_undo_redo(ReversableFunction::new(Funcs::Remove, idx, str));
                    }
                    else{
                        let str = self.document.remove(self.get_document_index(self.document.get_cursor()), 1);
                        self.document.push_to_undo_redo(ReversableFunction::new(Funcs::Remove, idx, str));
                    }
                    self.update_lines_lenghts();
                    self.update();
                }
                Key::Delete => {
                    let idx = self.get_document_index(self.document.get_cursor());
                    if idx >= self.document.get_length(){
                        continue;
                    }
                    let str = self.document.remove(idx, 1);
                    self.document.push_to_undo_redo(ReversableFunction::new(Funcs::Delete, self.get_document_index(self.document.get_cursor()), str));
                    self.update_lines_lenghts();
                    self.update();
                }
                Key::Char(ch)=> {
                    let idx = self.get_document_index(self.document.get_cursor());
                    if idx > self.document.get_length(){}
                    else{
                        self.document.push_to_undo_redo(ReversableFunction::new(Funcs::Insert, self.get_document_index(self.document.get_cursor()), ch.to_string()));
                        self.document.insert(self.get_document_index(self.document.get_cursor()), ch.to_string());
                        if ch == '\n'{
                            self.update_lines_lenghts();
                            self.document.get_cursor_mut().inc_y();
                        }
                        else{
                            // self.increment_lenght(self.cursor_pos.y - 1);
                            self.document.get_cursor_mut().inc_x();
                        }
                        
                    }
                    self.update();
                }
                _ => {}
            }
        }
    }
}

impl InsertMode{
    pub fn init(document: Document) -> InsertMode{
        InsertMode{
            document
        }
    }

    fn get_document_index(&self, cursor: &CursorPos) -> usize{
        let mut idx: usize = 0;
        for (i, line) in self.document.get_text().lines().enumerate(){
            if i == cursor.get_y() - 1{
                break
            } 
            idx += line.chars().count() + 1;
        }
        idx + cursor.get_x() - 1
    }

    fn get_cursor_from_index(index: usize, line_lengths: Vec<usize>) -> CursorPos{
        let mut index = index;
        let mut x = 1;
        let mut y = 1;
        for i in line_lengths.iter(){
            if index > *i{
                index -= i + 1;
                y += 1;
            }else{
                x = std::cmp::max(index, 1);
                break
            }
        }
        CursorPos::new(x, y)
    }

    // fn dec_x(&mut self){
        
    //     if self.cursor_pos.x == 1 && self.cursor_pos.y != 1{
    //         self.dec_y();
    //         self.cursor_pos.x = self.get_line_length(self.cursor_pos.y - 1) + 1;
    //         return
    //     }
    //     self.cursor_pos.x = max(self.cursor_pos.x - 1, 1); 
    // }

    // fn inc_x(&mut self){
    //     if self.cursor_pos.x == self.get_line_length(self.cursor_pos.y - 1) + 1 && self.cursor_pos.y != self.get_num_lines(){
    //         self.inc_y();
    //         self.cursor_pos.x = 1;
    //     }else{
    //         self.cursor_pos.x = min(self.cursor_pos.x + 1, self.get_line_length(self.cursor_pos.y - 1) + 1)
    //     }
    // }

    // fn inc_y(&mut self){
    //     if self.cursor_pos.y == self.get_num_lines(){
    //         return;
    //     } 
    //     self.cursor_pos.y += 1;
    // }

    // fn dec_y(&mut self){
    //     if self.cursor_pos.y == 1{
    //         return
    //     }
    //     self.cursor_pos.y -= 1;
    // }

    fn update_lines_lenghts(&mut self){
        self.document.recalculate_line_lenghts();
    }

    // fn increment_lenght(&mut self, line: usize){
    //     self.lines_manager.increment_lenght(line);
    // }

    // fn get_line_length(&self, line: usize) -> usize{
    //     self.lines_manager.get_line_lenght(line)
    // }

    // fn get_num_lines(&self) -> usize{
    //     self.lines_manager.get_num_lines()
    // }

    // fn get_line_lenght_vec(&self) -> Vec<usize>{
    //     self.lines_manager.get_line_lenght_vec()
    // }
}