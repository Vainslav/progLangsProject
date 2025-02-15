use crate::util::reversable_function::ReversableFunction;

use super::{cursor_manager::CursorPos, text_manager::TextManager};
use std::process::exit;

pub struct Document{
    text: TextManager,
    file_name: String,
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
            file_name: file_name,
        }
    }

    pub fn save(&self){
        std::fs::write(&self.file_name, self.text.get_all_text()).expect("Error writing to file");
    }

    pub fn change_file(&mut self, file_name: String){
        self.text.change_file(file_name);
    }



    pub fn insert(&mut self, index: usize, str: String){
        let old_cursor = self.get_cursor().to_owned();
        self.text.insert(index, str);
        self.text.update_offset(&old_cursor);
    }

    pub fn remove(&mut self, index: usize, length: usize) -> String{
        let old_cursor = self.get_cursor().to_owned();
        let string = self.text.remove(index, length);
        self.text.update_offset(&old_cursor);
        string
    }

    pub fn get_text_with_offset(&self) -> String{
        self.text.get_text_with_offset()
    }

    pub fn get_all_text(&self) -> String{
        self.text.get_all_text()
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
        let old_cursor = self.get_cursor().to_owned();
        self.text.undo();
        self.text.update_offset(&old_cursor);
        self.recalculate_line_lenghts();
    }

    pub fn redo(&mut self){
        let old_cursor = self.get_cursor().to_owned();
        self.text.redo();
        self.text.update_offset(&old_cursor);
        self.recalculate_line_lenghts();
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

    pub fn move_cursor_right(&mut self){
        let old_cursor = self.text.get_cursor().to_owned();
        self.text.inc_x();
        self.text.update_offset(&old_cursor);
    }

    pub fn move_cursor_down(&mut self){
        let old_cursor = self.text.get_cursor().to_owned();
        self.text.inc_y();
        self.text.update_offset(&old_cursor);
    }

    pub fn move_cursor_left(&mut self){
        let old_cursor = self.text.get_cursor().to_owned();
        self.text.dec_x();
        self.text.update_offset(&old_cursor);
    }

    pub fn move_cursor_up(&mut self){
        let old_cursor = self.text.get_cursor().to_owned();
        self.text.dec_y();
        self.text.update_offset(&old_cursor);
    }

    pub fn set_cursor_from_mouse_pos(&mut self, mouse: (u16, u16)){
        let offset = self.text.get_offset();
        let cursor = self.text.get_cursor_mut();
        cursor.set_x_display(mouse.0);
        cursor.set_y_display(mouse.1);
        cursor.set_x_actual(mouse.0 as usize + offset.0);
        cursor.set_y_actual(mouse.1 as usize + offset.1);
    }

    pub fn get_offset(&self) -> (usize, usize){
        self.text.get_offset()
    }

    pub fn set_cursor(&mut self, cursor: CursorPos){
        self.text.set_cursor(cursor);
    }
}