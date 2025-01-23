use std::fs::{read_to_string, File};

use super::text_manager::{self, TextManager};
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

    pub fn remove(&mut self, index: usize, length: usize){
        self.text.remove(index, length);
    }

    pub fn get_text(&self) -> String{
        self.text.get_text()
    }
}