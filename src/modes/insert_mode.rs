use std::fs::read_to_string;
use std::io::stdin;
use std::io::{Write, stdout};
use std::cmp::{min, max};


use termion::event::Key;
use termion::input::TermRead;
use termion::screen::IntoAlternateScreen;
use std::fs;
use termion::raw::IntoRawMode;

use crate::util::piece_table::PieceTable;
use crate::managers::lines_manager::LinesManager;
use crate::managers::undo_redo_manager::UndoRedoManager;
use crate::util::reversable_function::ReversableFunction;
use crate::util::reversable_function::Funcs;
use crate::modes::mode_trait::Mode;

struct CursorPos{
    x: usize,
    y: usize
}

pub struct InsertMode{
    document: PieceTable,
    cursor_pos: CursorPos,
    lines_manager: LinesManager,
    undo_redo: UndoRedoManager,
}

impl Mode for InsertMode{
    fn update(&self){
        print!("{}{}", termion::clear::All, termion::cursor::Goto(1,1));
        print!("{}", self.document.get_text().replace("\n", "\n\r"));
        print!("{}", termion::cursor::Goto(self.cursor_pos.x as u16, self.cursor_pos.y as u16))
    }

    fn run(&mut self){
        let mut stdout = stdout().into_raw_mode().unwrap().into_alternate_screen().unwrap();
        self.update();
        let stdin = stdin();
        for c in stdin.keys() {
            match c.unwrap() {
                Key::Ctrl('q') => {
                    fs::write("input_text", self.document.get_text()).expect("Unable to write file");
                    break;
                }
                Key::Ctrl('c') => {
                    break;
                }
                Key::Ctrl('z') => {
                    let function = self.undo_redo.undo();
                    if !function.is_none(){
                        let reversable_function = function.unwrap();
                        match reversable_function.func{
                            Funcs::Insert => {
                                self.document.remove(reversable_function.index, reversable_function.string.chars().count());
                                self.cursor_pos = Self::get_cursor_from_index(reversable_function.index, self.get_line_lenght_vec())
                            }
                            Funcs::Remove => {
                                self.document.insert({ 
                                    if reversable_function.index <= reversable_function.string.chars().count(){
                                        0
                                    }else{
                                        reversable_function.index - reversable_function.string.chars().count()
                                    }
                                }, reversable_function.string.clone());
                                self.cursor_pos = Self::get_cursor_from_index(reversable_function.index + reversable_function.string.chars().count(), self.get_line_lenght_vec())
                            }
                            Funcs::Delete => {
                                self.document.insert(reversable_function.index, reversable_function.string.clone());
                                self.cursor_pos = Self::get_cursor_from_index(reversable_function.index + reversable_function.string.chars().count(), self.get_line_lenght_vec())
                            }
                            _ => print!("Anlaki")
                        }
                    }
                    self.update_lines_lenghts();
                    self.update();
                }
                Key::Ctrl('y') => {
                    let function = self.undo_redo.redo();
                    if !function.is_none(){
                        let reversable_function = function.unwrap();
                        match reversable_function.func{
                            Funcs::Insert => {
                                self.document.insert(reversable_function.index, reversable_function.string.clone());
                                self.cursor_pos = Self::get_cursor_from_index(reversable_function.index, self.get_line_lenght_vec())
                            }
                            Funcs::Remove => {
                                self.document.remove({ 
                                    if reversable_function.index <= reversable_function.string.chars().count(){
                                        0
                                    }else{
                                        reversable_function.index - reversable_function.string.chars().count()
                                    }
                                }, reversable_function.string.chars().count());
                                self.cursor_pos = Self::get_cursor_from_index(reversable_function.index, self.get_line_lenght_vec());
                            }
                            Funcs::Delete => {
                                self.document.remove(reversable_function.index, reversable_function.string.chars().count());
                                self.cursor_pos = Self::get_cursor_from_index(reversable_function.index + reversable_function.string.chars().count(), self.get_line_lenght_vec())
                            }
                            _ => print!("Anlaki")
                        }
                    }
                    self.update_lines_lenghts();
                    self.update();
                }
                Key::Left => {
                    self.dec_x();
                    self.update();
                }
                Key::Right => {
                    self.inc_x();
                    self.update();
                }
                Key::Up => {
                    self.dec_y();
                    self.cursor_pos.x = min(self.cursor_pos.x, self.get_line_length(self.cursor_pos.y - 1) + 1);
                    self.update();
                }
                Key::Down => {
                    self.inc_y();
                    self.cursor_pos.x = min(self.cursor_pos.x, self.get_line_length(self.cursor_pos.y - 1) + 1);
                    self.update();
                }
                Key::Backspace => {
                    let idx = self.get_document_index(&self.cursor_pos);
                    if self.cursor_pos.x == 1 && self.cursor_pos.y == 1{
                        continue;
                    }
                    self.dec_x();
                    if self.cursor_pos.x == 1 {
                        self.undo_redo.push(ReversableFunction::new(Funcs::Remove, idx, self.document.remove(self.get_document_index(&self.cursor_pos), 1).unwrap()));
                    }
                    else{
                        self.undo_redo.push(ReversableFunction::new(Funcs::Remove, idx, self.document.remove(self.get_document_index(&self.cursor_pos), 1).unwrap()));
                    }
                    self.update_lines_lenghts();
                    self.update();
                }
                Key::Delete => {
                    let idx = self.get_document_index(&self.cursor_pos);
                    if idx >= self.document.get_length(){
                        continue;
                    }
                    self.undo_redo.push(ReversableFunction::new(Funcs::Delete, self.get_document_index(&self.cursor_pos), self.document.remove(idx, 1).unwrap()));
                    self.update_lines_lenghts();
                    self.update();
                }
                Key::Char(ch)=> {
                    let idx = self.get_document_index(&self.cursor_pos);
                    if idx > self.document.get_length(){}
                    else{
                        self.undo_redo.push(ReversableFunction::new(Funcs::Insert, self.get_document_index(&self.cursor_pos), ch.to_string()));
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
                    self.update();
                }
                _ => {}
            }
            stdout.flush().unwrap();
        }
    }
}

impl InsertMode{
    pub fn init(file: &str) -> InsertMode{
        let mut piece_table = PieceTable::new();
        let text_from_file = read_to_string(file).unwrap();
        let lines_manager = LinesManager::init(&text_from_file);
        piece_table.assign_buffer(text_from_file);
        InsertMode{
            document: piece_table,
            cursor_pos: CursorPos{
                x: 1,
                y: 1
            },
            lines_manager: lines_manager,
            undo_redo: UndoRedoManager::new(),
        }
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
        CursorPos{x,y}
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
        self.lines_manager.recalculate_line_lenghts(self.document.get_text());
    }

    fn increment_lenght(&mut self, line: usize){
        self.lines_manager.increment_lenght(line);
    }

    fn get_line_length(&self, line: usize) -> usize{
        self.lines_manager.get_line_lenght(line)
    }

    fn get_num_lines(&self) -> usize{
        self.lines_manager.get_num_lines()
    }

    fn get_line_lenght_vec(&self) -> Vec<usize>{
        self.lines_manager.get_line_lenght_vec()
    }
}