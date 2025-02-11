use std::fmt::write;
use std::io::stdin;
use std::thread;
use std::time::Duration;

use termion::color;
use termion::color::Color;
use termion::event::Event;
use termion::event::Key;
use termion::event::MouseButton;
use termion::event::MouseEvent;
use termion::input::MouseTerminal;
use termion::input::TermRead;
use termion::raw::RawTerminal;
use termion::terminal_size;

use std::io::Stdout;
use std::io::Write;

use crate::managers::document_manager::Document;
use crate::util::reversable_function::ReversableFunction;
use crate::util::reversable_function::Funcs;
use crate::managers::cursor_manager::CursorPos;

struct ColoringRule {
    pub start: usize,
    pub end: usize,
}

//termion::color::Bg(color::LightBlue)
fn update(stdout: &mut MouseTerminal<RawTerminal<Stdout>>, document: &mut Document, coloring: (usize, usize)){
    let terminal_size = terminal_size().unwrap();
    write!(stdout, "{}{}", termion::clear::All, termion::cursor::Goto(1,1)).unwrap();

    if coloring == (0,0){
        write!(stdout, "{}", document.get_text_with_offset().replace("\n", "\n\r")).unwrap();
    }else{
        print_text(stdout, document.get_text_with_offset(), coloring);
    }
    
    if terminal_size.1 >= 30 && terminal_size.0 >= 100{
        write!(stdout, "{}", termion::cursor::Goto(1, terminal_size.1)).unwrap();
        write!(stdout, "--INSERT--").unwrap();
        write!(stdout, "{}", termion::cursor::Goto(terminal_size.0 - 30, terminal_size.1)).unwrap();
        write!(stdout, "{} , {}", document.get_cursor().get_x_actual(), document.get_cursor().get_y_actual()).unwrap();
    }
    write!(stdout, "{}", termion::cursor::Goto(document.get_cursor().get_x_display(), document.get_cursor().get_y_display())).unwrap();
    stdout.flush().unwrap();
}

pub fn run(stdout: &mut MouseTerminal<RawTerminal<Stdout>>, document: &mut Document){
    update(stdout, document, (0,0));
    let mut events = stdin().events();
    let mut last_terminal_size = terminal_size().unwrap();

    let mut mouse_start: Option<usize> = None;
    let mut mouse_current: Option<usize> = None;
    let mut is_mouse_held: bool = false;

    loop{
        if !is_mouse_held{
            mouse_current = None;
            mouse_start = None;
        }

        let event = events.next()
        .map(|result| result.unwrap_or(Event::Unsupported(Vec::new())))
        .unwrap_or(Event::Unsupported(Vec::new()));

        match event {
            Event::Key(Key::Ctrl('q')) => {
                document.save();
                break;
            }
            Event::Key(Key::Ctrl('p')) => {
                break;
            }
            Event::Key(Key::Ctrl('z')) => {
                document.undo();
            }
            Event::Key(Key::Ctrl('y')) => {
                document.redo();
            }
            Event::Key(Key::Left) => {
                document.move_cursor_left();
            }
            Event::Key(Key::Right) => {
                document.move_cursor_right();
            }
            Event::Key(Key::Up) => {
                document.move_cursor_up();
            }
            Event::Key(Key::Down) => {
                document.move_cursor_down();
            }
            Event::Key(Key::Backspace) => {
                handle_backspace(document);
            }
            Event::Key(Key::Delete) => {
                handle_delete(document);
            }
            Event::Key(Key::Char(ch)) => {
                handle_char(document, ch);
            }
            Event::Mouse(me) => {
                match me {
                    MouseEvent::Press(MouseButton::Left, x, y) => {
                        mouse_start = Some(get_text_index(document.get_text_with_offset(), &CursorPos::new(x, y)));
                        is_mouse_held = true;
                        unsafe{
                            document.set_cursor(CursorPos::new(x, y));
                        }
                    }
                    MouseEvent::Hold(x, y) => {
                        mouse_current = Some(get_text_index(document.get_text_with_offset(), &CursorPos::new(x, y)));
                        unsafe{
                            document.set_cursor(CursorPos::new(x, y))
                        }
                    }
                    MouseEvent::Release(x, y) => {
                        if is_mouse_held{
                            is_mouse_held = false
                        }
                    }
                    _ => {}
                }
            }
            _ => {}
        }
        if mouse_start.is_some() && mouse_current.is_some(){
            let first_index = mouse_start.unwrap();
            let second_index = mouse_current.unwrap();
            update(stdout, document, (first_index, second_index));
        }else{
            update(stdout, document, (0,0));
        }
    }
}

fn get_text_index(text: String, cursor: &CursorPos) -> usize{
    let mut idx: usize = 0;
    for (i, line) in text.lines().enumerate(){
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

fn handle_backspace(document: &mut Document){
    let idx = get_text_index(document.get_all_text(), document.get_cursor());
    if document.get_cursor().get_x_actual() == 1 && document.get_cursor().get_y_actual() == 1{
        return
    }
    document.move_cursor_left();

    let str = document.remove(get_text_index(document.get_all_text(), document.get_cursor()), 1);

    let mut new_cursor = document.get_cursor().clone();
    new_cursor.inc_x();
    document.push_to_undo_redo(ReversableFunction::new(
        Funcs::Remove, 
        idx, 
        str,
        new_cursor
    ));
    document.recalculate_line_lenghts();
}

fn handle_delete(document: &mut Document){
    let idx = get_text_index(document.get_all_text(), document.get_cursor());
    if idx >= document.get_length(){
        return
    }
    let str = document.remove(idx, 1);
    document.push_to_undo_redo(ReversableFunction::new(
        Funcs::Delete,
        idx,
        str,
        document.get_cursor().clone()
    ));
    document.recalculate_line_lenghts();
}

fn handle_char(document: &mut Document, ch: char){
    let idx = get_text_index(document.get_all_text(), document.get_cursor());
    if idx > document.get_length(){}
    else{
        document.push_to_undo_redo(ReversableFunction::new(
            Funcs::Insert, 
            get_text_index(document.get_all_text(), document.get_cursor()), 
            ch.to_string(),
            document.get_cursor().clone()
        ));
        document.insert(get_text_index(document.get_all_text(), document.get_cursor()), ch.to_string());
        if ch == '\n'{
            document.recalculate_line_lenghts();
            document.get_cursor_mut().set_max_newline();
            document.move_cursor_down();
        }
        else{
            // increment_lenght(cursor_pos.y - 1);
            document.move_cursor_right();
        }
    }
}

fn print_text(stdout: &mut MouseTerminal<RawTerminal<Stdout>>, text: String, coloring: (usize, usize)){
    let mut until_first: Vec<char> = Vec::new();
    let mut change_color: Vec<char> = Vec::new();
    let mut after_change_color: Vec<char> = Vec::new();
    let _ =text.chars()
            .enumerate()
            .for_each(|(i, c)| -> () {
                if i < std::cmp::min(coloring.0, coloring.1){
                    until_first.push(c);
                }else if i >= std::cmp::max(coloring.0, coloring.1){
                    after_change_color.push(c);
                }else{
                    change_color.push(c);
                }
            });
    write!(stdout,"{}",until_first.iter().collect::<String>().replace("\n", "\n\r")).unwrap();
    write!(stdout,"{}{}{}",termion::color::Bg(termion::color::LightBlue), change_color.iter().collect::<String>().replace("\n", "\n\r"), termion::color::Bg(termion::color::Reset)).unwrap();
    write!(stdout,"{}",after_change_color.iter().collect::<String>().replace("\n", "\n\r")).unwrap();
}
