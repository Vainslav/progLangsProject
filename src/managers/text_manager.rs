use std::fs::read_to_string;
use std::io::stdin;
use std::io::{Write, stdout, Error};
use std::cmp::{min, max};

use crate::util::piece_table::PieceTable;
use crate::managers::lines_manager::LinesManager;
use crate::managers::undo_redo_manager::UndoRedoManager;
use crate::util::reversable_function::Funcs;
use crate::util::reversable_function::ReversableFunction;

use super::cursor_manager::CursorPos;

pub struct TextManager{
    text: PieceTable,
    lines_manager: LinesManager,
    cursor: CursorPos,
    undo_redo: UndoRedoManager,
}


impl TextManager{
    pub fn init(file_name: &str) -> Result<TextManager, Error>{
        let mut piece_table = PieceTable::new();
        let text_from_file = read_to_string(file_name)?;
        let lines_manager = LinesManager::init(&text_from_file);
        piece_table.assign_buffer(text_from_file);
        Ok(TextManager{
            text: piece_table,
            lines_manager: lines_manager,
            cursor: CursorPos::new(1, 1),
            undo_redo: UndoRedoManager::new(),
        })
    }

    pub fn get_text(&self) -> String{
        self.text.get_text()
    }

    pub fn insert(&mut self, index: usize, str: String){
        self.text.insert(index, str);
        self.update_lines_lenghts();
    }

    pub fn remove(&mut self, index: usize, length: usize) -> String{
        let ret: String =  self.text.remove(index, length).unwrap();
        self.update_lines_lenghts();
        ret
    }

    pub fn update_lines_lenghts(&mut self){
        self.lines_manager.recalculate_line_lenghts(self.text.get_text());
    }

    pub fn change_file(&mut self, file_name: String){
        self.text.assign_buffer(file_name);
        self.update_lines_lenghts();
    }

    pub fn get_length(&self) -> usize{
        self.text.get_length()
    }

    pub fn get_line_length(&self, line: usize) -> usize{
        self.lines_manager.get_line_lenght(line)
    }

    pub fn get_num_lines(&self) -> usize{
        self.lines_manager.get_num_lines()
    }

    pub fn undo(&mut self){
        let function = self.undo_redo.undo();
        let mut function_cursor:Option<&CursorPos> = None;
        if function.is_some(){
            let reversable_function = function.unwrap();
            function_cursor = Some(reversable_function.get_cursor());
            let text = &self.text.get_text();
            match reversable_function.get_func(){
                Funcs::Insert => {
                    self.text.remove(*reversable_function.get_index(), reversable_function.get_string().chars().count());
                }
                Funcs::Remove => {
                    self.text.insert({ 
                        if reversable_function.get_index() <= &reversable_function.get_string().chars().count(){
                            0
                        }else{
                            reversable_function.get_index() - reversable_function.get_string().chars().count()
                        }
                    }, reversable_function.get_string().clone());
                    self.lines_manager.recalculate_line_lenghts(self.text.get_text());
                }
                Funcs::Delete => {
                    self.text.insert(*reversable_function.get_index(), reversable_function.get_string().clone());
                    self.lines_manager.recalculate_line_lenghts(self.text.get_text());
                }
            }
            self.cursor.set_x(reversable_function.get_cursor().get_x());
            self.cursor.set_y(reversable_function.get_cursor().get_y());
        }
        
        self.lines_manager.recalculate_line_lenghts(self.text.get_text());
        if function_cursor.is_some(){
            let new_cursor = function_cursor.unwrap();
            
        }
    }

    pub fn redo(&mut self){
        let function = self.undo_redo.redo();
        let mut function_cursor:Option<&CursorPos> = None;
        if !function.is_none(){
            let text = &self.text.get_text();
            let reversable_function = function.unwrap();
            function_cursor = Some(reversable_function.get_cursor());
            match reversable_function.get_func(){
                Funcs::Insert => {
                    self.text.insert(*reversable_function.get_index(), reversable_function.get_string().clone());
                }
                Funcs::Remove => {
                    self.text.remove({ 
                        if reversable_function.get_index() <= &reversable_function.get_string().chars().count(){
                            0
                        }else{
                            reversable_function.get_index() - reversable_function.get_string().chars().count()
                        }
                    }, reversable_function.get_string().chars().count());
                }
                Funcs::Delete => {
                    self.text.remove(*reversable_function.get_index(), reversable_function.get_string().chars().count());
                }
            }
            self.cursor.set_x(reversable_function.get_cursor().get_x());
            self.cursor.set_y(reversable_function.get_cursor().get_y());
        }
        self.lines_manager.recalculate_line_lenghts(self.text.get_text());
        if function_cursor.is_some(){
            let new_cursor = function_cursor.unwrap();
            // self.cursor.set_x(new_cursor.get_x());
            // self.cursor.set_y(new_cursor.get_y());
        }
    }

    pub fn push_to_undo_redo(&mut self, func: ReversableFunction){
        self.undo_redo.push(func);
    }



    pub fn get_cursor(&self) -> &CursorPos{
        &self.cursor
    }

    pub fn get_cursor_mut(&mut self) -> &mut CursorPos{
        &mut self.cursor
    }

    pub fn inc_x(&mut self){
        if self.cursor.get_x() == self.get_line_length(self.cursor.get_y() - 1) + 1 && self.cursor.get_y() != self.get_num_lines(){
            self.inc_y();
            self.cursor.set_x(1);
        }else{
            self.cursor.set_x(min(self.cursor.get_x() + 1, self.get_line_length(self.cursor.get_y() - 1) + 1));
        }
        self.cursor.update_max();
    }

    pub fn inc_y(&mut self){
        if self.cursor.get_y() == self.get_num_lines(){
            return;
        } 
        self.cursor.inc_y();
        self.cursor.set_x(min(self.cursor.get_max(), self.get_line_length(self.cursor.get_y() - 1) + 1));
    }

    pub fn dec_x(&mut self){
        if self.cursor.get_x() == 1 && self.cursor.get_y() != 1{
            self.dec_y();
            self.cursor.set_x(self.get_line_length(self.cursor.get_y() - 1) + 1);
            return
        }
        self.cursor.set_x(max(self.cursor.get_x() - 1, 1));
        self.cursor.update_max();
    }

    pub fn dec_y(&mut self){
        if self.cursor.get_y() == 1{
            return
        }
        self.cursor.dec_y();
        self.cursor.set_x(min(self.cursor.get_max(), self.get_line_length(self.cursor.get_y() - 1) + 1));
    }
}

fn find_cursor_position(text: &str, index: usize) -> Option<CursorPos> {
    let mut x = 1;
    let mut y = 1;
    let mut last_newline_index = 0;

    for (i, c) in text.chars().enumerate() {
        if i == index {
            x = i - last_newline_index;
            return Some(CursorPos::new(x, y));
        }
        if c == '\n' {
            y += 1;
            last_newline_index = i + 1;
        }
    }

    if index == text.len() {
        x = text.len() - last_newline_index;
        return Some(CursorPos::new(x, y));
    }

    None
}