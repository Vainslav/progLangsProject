use std::thread;
use std::time::Duration;

use termion::event::Key;
use termion::input::TermRead;
use termion::raw::RawTerminal;
use termion::terminal_size;

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

// static mut CNT:u16 = 0; //debug

impl Mode for InsertMode{
    fn update(&self, stdout: &mut RawTerminal<Stdout>){
        let terminal_size = terminal_size().unwrap();
        write!(stdout, "{}{}", termion::clear::All, termion::cursor::Goto(1,1)).unwrap();
        write!(stdout, "{}", self.document.get_text().replace("\n", "\n\r")).unwrap();
        if terminal_size.1 >= 30 && terminal_size.0 >= 100{
            write!(stdout, "{}", termion::cursor::Goto(1, terminal_size.1)).unwrap();
            write!(stdout, "--INSERT--").unwrap();
            write!(stdout, "{}", termion::cursor::Goto(terminal_size.0 - 30, terminal_size.1)).unwrap();
            write!(stdout, "{} , {}", self.document.get_cursor().get_x_actual(), self.document.get_cursor().get_y_actual()).unwrap();
            write!(stdout, "[{} , {}]", terminal_size.0, terminal_size.1).unwrap();
            // unsafe{
            //     write!(stdout, "[{}]", CNT).unwrap(); // debug
            //     CNT += 1
            // }
        }
        write!(stdout, "{}", termion::cursor::Goto(self.document.get_cursor().get_x_display(), self.document.get_cursor().get_y_display())).unwrap();
        stdout.flush().unwrap();
    }

    fn run(&mut self, stdout: &mut RawTerminal<Stdout>){
        self.update(stdout);
        let mut keys = termion::async_stdin().keys();
        let mut last_terminal_size = terminal_size().unwrap();
        loop{
            if last_terminal_size != terminal_size().unwrap() {
                last_terminal_size = terminal_size().unwrap();
                
                self.update(stdout);
            }
            match keys.next() {
                Some(c) => {
                    match c {
                        Ok(c) => {
                            match c {
                                Key::Ctrl('q') => {
                                    self.document.save();
                                    break;
                                }
                                Key::Ctrl('c') => {
                                    break;
                                }
                                Key::Ctrl('z') => {
                                    self.document.undo();
                                    self.update(stdout);
                                }
                                Key::Ctrl('y') => {
                                    self.document.redo();
                                    self.update(stdout);
                                }
                                Key::Left => {
                                    self.document.move_cursor_left();
                                    self.update(stdout);
                                }
                                Key::Right => {
                                    self.document.move_cursor_right();
                                    self.update(stdout);
                                }
                                Key::Up => {
                                    self.document.move_cursor_up();
                                    self.update(stdout);
                                }
                                Key::Down => {
                                    self.document.move_cursor_down();
                                    self.update(stdout);
                                }
                                Key::Backspace => {
                                    let idx = self.get_document_index(self.document.get_cursor());
                                    if self.document.get_cursor().get_x_actual() == 1 && self.document.get_cursor().get_y_actual() == 1{
                                        continue;
                                    }
                                    self.document.move_cursor_left();

                                    let str = self.document.remove(self.get_document_index(self.document.get_cursor()), 1);

                                    let mut new_cursor = self.document.get_cursor().clone();
                                    new_cursor.inc_x();
                                    self.document.push_to_undo_redo(ReversableFunction::new(
                                        Funcs::Remove, 
                                        idx, 
                                        str,
                                        new_cursor
                                    ));
                                    self.update_lines_lenghts();
                                    self.update(stdout);
                                }
                                Key::Delete => {
                                    let idx = self.get_document_index(self.document.get_cursor());
                                    if idx >= self.document.get_length(){
                                        continue;
                                    }
                                    let str = self.document.remove(idx, 1);
                                    self.document.push_to_undo_redo(ReversableFunction::new(
                                        Funcs::Delete,
                                        idx,
                                        str,
                                        self.document.get_cursor().clone()
                                    ));
                                    self.update_lines_lenghts();
                                    self.update(stdout);
                                }
                                Key::Char(ch) => {
                                    let idx = self.get_document_index(self.document.get_cursor());
                                    if idx > self.document.get_length(){}
                                    else{
                                        self.document.push_to_undo_redo(ReversableFunction::new(
                                            Funcs::Insert, 
                                            self.get_document_index(self.document.get_cursor()), 
                                            ch.to_string(),
                                            self.document.get_cursor().clone()
                                        ));
                                        self.document.insert(self.get_document_index(self.document.get_cursor()), ch.to_string());
                                        if ch == '\n'{
                                            self.update_lines_lenghts();
                                            self.document.get_cursor_mut().set_max_newline();
                                            self.document.move_cursor_down();
                                        }
                                        else{
                                            // self.increment_lenght(self.cursor_pos.y - 1);
                                            self.document.move_cursor_right();
                                        }
                                    }
                                    self.update(stdout);
                                }
                                _ => {}
                            }
                        }
                        Err(_) => {}                        
                    }
                }
                None => {}
            }
            thread::sleep(Duration::from_millis(10));
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
        for (i, line) in self.document.get_all_text().lines().enumerate(){
            if i == cursor.get_y_actual() - 1{
                break
            } 
            idx += line.chars().count() + 1;
        }
        idx + cursor.get_x_actual() - 1
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
        CursorPos::new(x as u16, y)
    }

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