use termion::{event::{Event, Key, MouseButton, MouseEvent}, input::MouseTerminal, raw::IntoRawMode, screen::{AlternateScreen, IntoAlternateScreen}, terminal_size};

use std::{fs::DirEntry, io::{stdout, Write}, path::PathBuf};

use crate::{event_lisner::get_event_reviever, managers::cursor_manager::CursorPos};

fn update(stdout: &mut MouseTerminal<termion::raw::RawTerminal<std::io::Stdout>>, cur_path: &String, number_of_skips: usize){
    let terminal_size = terminal_size().unwrap();

    write!(stdout, "{}{}", termion::clear::All, termion::cursor::Goto(1,1)).unwrap();
    let mut dirs:Vec<String> = vec![];
    let mut files: Vec<String> = vec![];

    std::fs::read_dir(cur_path).unwrap().skip(number_of_skips).take(terminal_size.1 as usize - 2).for_each(|path| {
        let path_unwraped = path.unwrap();
        if path_unwraped.file_type().unwrap().is_dir(){
            dirs.push(path_unwraped.file_name().into_string().unwrap());
        }else{
            files.push(path_unwraped.file_name().into_string().unwrap());
        }
    });

    let mut line: u16 = 2;

    dirs.sort();
    files.sort();

    write!(stdout, "../\n{}", termion::cursor::Goto(1, line)).unwrap();
    line += 1;
    dirs.iter().for_each(|dir|{
        write!(stdout, "{}/\n{}", dir, termion::cursor::Goto(1, line)).unwrap();
        line += 1;
    });
    files.iter().for_each(|file|{
        write!(stdout, "{}\n{}", file, termion::cursor::Goto(1, line)).unwrap();
        line += 1;
    });
    stdout.flush().unwrap();
}

pub fn choose_file() -> String{
    let mut stdout = MouseTerminal::from(stdout().into_raw_mode().unwrap());
    let command_listener = get_event_reviever();

    let mut command_message = String::new();

    let mut cur_dir = Some(std::fs::canonicalize(".").unwrap());

    let mut window_size = terminal_size().unwrap();

    let mut number_of_skips = 0;

    update(stdout.by_ref(), &cur_dir.to_owned().unwrap().into_os_string().into_string().unwrap(), number_of_skips);

    loop {
        if window_size != terminal_size().unwrap(){
            window_size = terminal_size().unwrap();
            update(stdout.by_ref(), &cur_dir.to_owned().unwrap().into_os_string().into_string().unwrap(), number_of_skips);
        }

        match command_listener.try_recv(){
            Ok(event) => {
                match event {
                    Event::Mouse(MouseEvent::Press(MouseButton::Left, x, y)) => {
                        if y == 1{
                            cur_dir = match cur_dir.as_ref().unwrap().parent(){
                                Some(path) => {Some(path.to_path_buf())},
                                None => {cur_dir},
                            };
                        }else{
                            let vec = read_dir_into_sorted_vec(&cur_dir.to_owned().unwrap().into_os_string().into_string().unwrap());
                            let file = match vec.iter().skip(number_of_skips + y as usize - 2).peekable().peek(){
                                Some(dir) => dir.to_owned(),
                                None => continue,
                            };
                            if file.file_type().unwrap().is_dir(){
                                cur_dir = Some(file.path());
                            }else{
                                return file.path().into_os_string().into_string().unwrap()
                            }
                        }
                        update(&mut stdout, &cur_dir.to_owned().unwrap().into_os_string().into_string().unwrap(), number_of_skips);
                    }
                    Event::Key(Key::Backspace) => {
                        command_message.pop();
                    }
                    Event::Key(Key::Ctrl('c')) => {
                        return "".to_string();
                    }
                    Event::Key(Key::Char(ch)) => {
                        command_message.push(ch);
                    }
                    _ => {}
                }
            }
            Err(_) => {}
        }
    }
}

fn read_dir_into_sorted_vec(cur_dir: &String) -> Vec<DirEntry>{
    let mut dirs:Vec<DirEntry> = vec![];
    let mut files: Vec<DirEntry> = vec![];

    std::fs::read_dir(cur_dir).unwrap().for_each(|path| {
        let path_unwraped = path.unwrap();
        if path_unwraped.file_type().unwrap().is_dir(){
            dirs.push(path_unwraped);
        }else{
            files.push(path_unwraped);
        }
    });

    dirs.sort_by(|path, path2| path.file_name().into_string().unwrap().partial_cmp(&path2.file_name().into_string().unwrap()).unwrap());
    files.sort_by(|path, path2| path.file_name().into_string().unwrap().partial_cmp(&path2.file_name().into_string().unwrap()).unwrap());

    dirs.append(&mut files);
    dirs
}