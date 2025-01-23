use std::fs::read_to_string;
use std::io::stdin;
use std::io::{Write, stdout, Error};
use std::cmp::{min, max};

use crate::util::piece_table::PieceTable;
use crate::managers::lines_manager::LinesManager;
use crate::managers::undo_redo_manager::UndoRedoManager;

pub struct TextManager{
    text: PieceTable,
    lines_manager: LinesManager,
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

    pub fn remove(&mut self, index: usize, length: usize){
        self.text.remove(index, length);
        self.update_lines_lenghts();
    }

    pub fn update_lines_lenghts(&mut self){
        self.lines_manager.recalculate_line_lenghts(self.text.get_text());
    }

    pub fn change_file(&mut self, file_name: String){
        self.text.assign_buffer(file_name);
    }
}