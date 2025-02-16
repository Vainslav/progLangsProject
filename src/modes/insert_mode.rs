use std::io::stdin;
use std::cmp::min;
use std::cmp::max;


use std::thread;
use std::sync::mpsc;
use std::time::Duration;

use termion::event;
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
        write!(stdout, "[{},{}]", document.get_offset().0, document.get_offset().1).unwrap();
        write!(stdout, "[{},{}]", coloring.0, coloring.1).unwrap();
    }
    write!(stdout, "{}", termion::cursor::Goto(document.get_cursor().get_x_display(), document.get_cursor().get_y_display())).unwrap();
    stdout.flush().unwrap();
}

pub fn run(stdout: &mut MouseTerminal<RawTerminal<Stdout>>, document: &mut Document){
    update(stdout, document, (0,0));
    let mut _last_terminal_size = terminal_size().unwrap();

    let mut highlight_start: Option<CursorPos> = None;
    let mut highlight_current: Option<CursorPos> = None;

    let mut window_size = terminal_size().unwrap();
    
    let (tx, rx) = mpsc::channel::<Event>();

    thread::spawn(move || {
        let mut events = stdin().events();

        loop{
            let event = events.next()
            .map(|result| result.unwrap_or(Event::Unsupported(Vec::new())))
            .unwrap_or(Event::Unsupported(Vec::new()));
            tx.send(event).unwrap()
        }
    });

    loop{
        if window_size != terminal_size().unwrap(){
            window_size = terminal_size().unwrap();
            if highlight_start.is_some() && highlight_current.is_some(){
                let first_index = get_text_index_display(document.get_text_with_offset(), highlight_start.as_ref().unwrap());
                let second_index = get_text_index_display(document.get_text_with_offset(), highlight_current.as_ref().unwrap());
                update(stdout, document, (first_index, second_index));
            }else{
                update(stdout, document, (0,0));
            }
        }

        match rx.try_recv(){
            Ok(event) => {
                match event {
                    Event::Key(Key::Ctrl('q')) => {
                        document.save();
                        write!(stdout, "{}", termion::cursor::Show).unwrap();
                        break;
                    }
                    Event::Key(Key::Ctrl('p')) => {
                        write!(stdout, "{}", termion::cursor::Show).unwrap();
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
                        handle_backspace(document, highlight_start,  highlight_current);
                    }
                    Event::Key(Key::Delete) => {
                        handle_delete(document, highlight_start, highlight_current);
                    }
                    Event::Key(Key::Char(ch)) => {
                        handle_char(document, ch, highlight_start, highlight_current);
                    }
                    Event::Mouse(me) => {
                        match me {
                            MouseEvent::Press(MouseButton::Left, x, y) => {
                                highlight_current = None;
                                document.set_cursor_from_mouse_pos(&mut (x, y));
                                highlight_start = Some(document.get_cursor().to_owned());
                            }
                            MouseEvent::Press(MouseButton::WheelDown, x, y) => {
                                if !document.is_offset_saved(){
                                    document.save_offset();
                                }
                                let document_offset = document.get_offset();
                                document.set_offset((document_offset.0, min(document_offset.1 + 1, (document.get_num_lines() as isize - terminal_size().unwrap().1 as isize).abs() as usize)));
                                write!(stdout, "{}", termion::cursor::Hide).unwrap();
                            }
                            MouseEvent::Press(MouseButton::WheelUp, x, y) => {
                                if !document.is_offset_saved(){
                                    document.save_offset();
                                }
                                let document_offset = document.get_offset();
                                if document_offset.1 == 0{
                                
                                }else{
                                    document.set_offset((document_offset.0, document_offset.1 - 1));
                                    write!(stdout, "{}", termion::cursor::Hide).unwrap();
                                }
                            }
                            MouseEvent::Hold(x, y) => {
                                document.set_cursor_from_mouse_pos(&mut (x, y));
                                if highlight_start.as_ref().unwrap() != document.get_cursor(){
                                    highlight_current = Some(document.get_cursor().to_owned());
                                }
                            }
                            _ => {}
                        }
                    }
                    _ => {}
                }
                
                match event{
                    Event::Mouse(me) => {
                        match me {
                            MouseEvent::Press(MouseButton::Left, _x, _y) => {
                                write!(stdout, "{}", termion::cursor::Show).unwrap();
                                document.save_offset();
                            }
                            MouseEvent::Hold(_x, _y) => {
                                document.save_offset();
                            }
                            _ => {}
                        }
                    }
                    _ => {
                        highlight_current = None;
                        highlight_start = None;
                        if document.is_offset_saved(){
                            document.reset_offset();
                            write!(stdout, "{}", termion::cursor::Show).unwrap();
                        }
                    }
                }
        
                if highlight_start.is_some() && highlight_current.is_some(){
                    let first_index = get_text_index_display(document.get_text_with_offset(), highlight_start.as_ref().unwrap());
                    let second_index = get_text_index_display(document.get_text_with_offset(), highlight_current.as_ref().unwrap());
                    update(stdout, document, (first_index, second_index));
                }else{
                    update(stdout, document, (0,0));
                }
            }
            Err(_) => {}
        }
    }
}

fn get_text_index_display(text: String, cursor: &CursorPos) -> usize{
    let mut idx: usize = 0;
    for (i, line) in text.lines().enumerate(){
        if i == cursor.get_y_display() as usize - 1{
            break
        } 
        idx += line.chars().count() + 1;
    }
    idx + cursor.get_x_display() as usize - 1
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

fn _get_cursor_from_index(index: usize, line_lengths: Vec<usize>) -> CursorPos{
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

fn handle_backspace(document: &mut Document, highlight_start: Option<CursorPos>, highlight_current: Option<CursorPos>){
    if highlight_start.is_some() && highlight_current.is_some(){
        if remove_chunk(document, highlight_start, highlight_current){
            return
        }
    }

    if document.get_cursor().get_x_actual() == 1 && document.get_cursor().get_y_actual() == 1{
        return
    }

    document.move_cursor_left();

    let idx = get_text_index(document.get_all_text(), document.get_cursor());

    let str = document.remove(idx, 1);
    
    let mut old_cursor = document.get_cursor().clone();

    old_cursor.inc_x();

    document.recalculate_line_lenghts();

    document.push_to_undo_redo(ReversableFunction::new(
        Funcs::Remove, 
        idx, 
        str,
        old_cursor
    ));
}

fn handle_delete(document: &mut Document, highlight_start: Option<CursorPos>, highlight_current: Option<CursorPos>){
    if highlight_start.is_some() && highlight_current.is_some(){
        if remove_chunk(document, highlight_start, highlight_current){
            return
        }
    }

    let idx = get_text_index(document.get_all_text(), document.get_cursor());

    if idx >= document.get_length(){
        return
    }

    let str = document.remove(idx, 1);
    document.push_to_undo_redo(ReversableFunction::new(
        Funcs::Delete,
        idx,
        str,
        document.get_cursor().to_owned()
    ));
    document.recalculate_line_lenghts();
}

fn handle_char(document: &mut Document, ch: char, highlight_start: Option<CursorPos>, highlight_current: Option<CursorPos>){
    if highlight_current.is_some() && highlight_start.is_some(){
        remove_chunk(document, highlight_start, highlight_current);
    }
    let idx = get_text_index(document.get_all_text(), document.get_cursor());
    if idx > document.get_length(){}
    else{
        document.push_to_undo_redo(ReversableFunction::new(
            Funcs::Insert, 
            idx, 
            {
            if ch != '\t'{
                ch.to_string()
            }else{
                "    ".to_string()
            }
            },
            document.get_cursor().to_owned()
        ));
        if ch == '\t'{
            document.insert(idx, "    ".to_string());
        }else{
            document.insert(idx, ch.to_string());
        }
        document.recalculate_line_lenghts();
        if ch == '\n'{
            document.get_cursor_mut().set_max_newline();
            document.move_cursor_down();
        }
        else if ch == '\t'{
            for _i in 0..4{
                document.move_cursor_right();
            }
        }
        else{
            document.move_cursor_right();
        }
    }
}

fn remove_chunk(document: &mut Document, chunk_start: Option<CursorPos>, chunk_end: Option<CursorPos>) -> bool{
    let start_index = get_text_index(document.get_all_text(), chunk_start.as_ref().unwrap());
    let end_index = get_text_index(document.get_all_text(), chunk_end.as_ref().unwrap());

    if start_index == end_index{
        return false
    }

    let idx = min(start_index, end_index);

    let str = document.remove(idx, max(start_index, end_index) - min(start_index, end_index));

    let old_cursor: CursorPos = {
        if start_index < end_index{
            chunk_start.unwrap()
        }else{
            chunk_end.unwrap()
        }
    };

    if start_index > end_index{

    }else{
        for _i in 0..str.len(){
            document.move_cursor_left();
        }
    }

    document.recalculate_line_lenghts();

    document.set_cursor(old_cursor.to_owned());

    document.push_to_undo_redo(ReversableFunction::new(
        Funcs::Remove,
        idx,
        str,
        old_cursor
    ));
    return true
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