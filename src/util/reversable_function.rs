use crate::managers::cursor_manager::CursorPos;

#[derive(PartialEq)]
pub enum Funcs{
    Insert,
    Remove,
    Delete,
}

pub struct ReversableFunction{
    func: Funcs,
    index: usize,
    string: String,
    cursor: CursorPos
}

impl ReversableFunction {
    pub fn new(fun: Funcs, idx: usize, str: String, cursor: CursorPos) -> ReversableFunction{
        ReversableFunction{
            func: fun,
            index: idx,
            string: str,
            cursor: cursor
        }
    }

    pub fn get_cursor(&self) -> &CursorPos{
        &self.cursor
    }

    pub fn get_index(&self) -> &usize{
        &self.index
    }

    pub fn get_func(&self) -> &Funcs{
        &self.func
    }

    pub fn get_string(&self) -> &String{
        &self.string
    }
}