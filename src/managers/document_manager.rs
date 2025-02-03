use crate::util::reversable_function::ReversableFunction;

use super::{cursor_manager::CursorPos, text_manager::TextManager};
use std::process::exit;

pub struct Document{
    text: TextManager,
    file_name: String
}

impl Document {
    pub fn new(file_name: String) -> Self{
        let text = match TextManager::init(&file_name){
            Ok(text_manager) => text_manager,
            Err(_) => {
                print!("Can't construct document");
                exit(1)
            }
        };
        Document{
            text: text,
            file_name: file_name
        }
    }

    pub fn save(&self){
        std::fs::write(&self.file_name, self.text.get_text()).expect("Error writing to file");
    }

    pub fn change_file(&mut self, file_name: String){
        self.text.change_file(file_name);
    }



    pub fn insert(&mut self, index: usize, str: String){
        self.text.insert(index, str);
    }

    pub fn remove(&mut self, index: usize, length: usize) -> String{
        self.text.remove(index, length)
    }

    pub fn get_text(&self) -> String{
        self.text.get_text()
    }



    pub fn recalculate_line_lenghts(&mut self){
        self.text.update_lines_lenghts();
    }

    pub fn get_length(&self) -> usize{
        self.text.get_length()
    }

    pub fn get_line_length(&self, line: usize) -> usize{
        self.text.get_line_length(line)
    }

    pub fn get_num_lines(&self) -> usize{
        self.text.get_num_lines()
    }



    pub fn undo(&mut self){
        self.text.undo();
    }

    pub fn redo(&mut self){
        self.text.redo();
    }

    pub fn push_to_undo_redo(&mut self, func: ReversableFunction){
        self.text.push_to_undo_redo(func);
    }



    pub fn get_cursor(&self) -> &CursorPos{
        self.text.get_cursor()
    }

    pub fn get_cursor_mut(&mut self) -> &mut CursorPos{
        self.text.get_cursor_mut()
    }

    pub fn inc_x(&mut self){
        self.text.inc_x();
    }

    pub fn inc_y(&mut self){
        self.text.inc_y();
    }

    pub fn dec_x(&mut self){
        self.text.dec_x();
    }

    pub fn dec_y(&mut self){
        self.text.dec_y();
    }
}