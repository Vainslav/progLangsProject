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
        if !function.is_none(){
            let reversable_function = function.unwrap();
            match reversable_function.func{
                Funcs::Insert => {
                    self.text.remove(reversable_function.index, reversable_function.string.chars().count());
                    // self.cursor = Self::get_cursor_from_index(reversable_function.index, self.get_line_lenght_vec())
                }
                Funcs::Remove => {
                    self.text.insert({ 
                        if reversable_function.index <= reversable_function.string.chars().count(){
                            0
                        }else{
                            reversable_function.index - reversable_function.string.chars().count()
                        }
                    }, reversable_function.string.clone());
                    // self.cursor_pos = Self::get_cursor_from_index(reversable_function.index + reversable_function.string.chars().count(), self.get_line_lenght_vec())
                }
                Funcs::Delete => {
                    self.text.insert(reversable_function.index, reversable_function.string.clone());
                    // self.cursor_pos = Self::get_cursor_from_index(reversable_function.index + reversable_function.string.chars().count(), self.get_line_lenght_vec())
                }
            }
        }
        self.update_lines_lenghts();
    }

    pub fn redo(&mut self){
        let function = self.undo_redo.redo();
        if !function.is_none(){
            let reversable_function = function.unwrap();
            match reversable_function.func{
                Funcs::Insert => {
                    self.text.insert(reversable_function.index, reversable_function.string.clone());
                    // self.cursor = Self::get_cursor_from_index(reversable_function.index, self.get_line_lenght_vec())
                }
                Funcs::Remove => {
                    self.text.remove({ 
                        if reversable_function.index <= reversable_function.string.chars().count(){
                            0
                        }else{
                            reversable_function.index - reversable_function.string.chars().count()
                        }
                    }, reversable_function.string.chars().count());
                    // self.cursor = Self::get_cursor_from_index(reversable_function.index, self.get_line_lenght_vec());
                }
                Funcs::Delete => {
                    self.text.remove(reversable_function.index, reversable_function.string.chars().count());
                    // self.cursor = Self::get_cursor_from_index(reversable_function.index + reversable_function.string.chars().count(), self.get_line_lenght_vec())
                }
                _ => print!("Anlaki")
            }
        }
        self.update_lines_lenghts();
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
}