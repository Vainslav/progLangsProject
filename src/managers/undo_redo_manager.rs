use crate::util::functions::{Funcs, Function};

pub struct UndoRedoManager {
    stack: Vec<Function>,
    pointer: i64,
}

impl UndoRedoManager {
    pub fn new() -> UndoRedoManager {
        UndoRedoManager {
            stack: Vec::new(),
            pointer: -1,
        }
    }

    pub fn push(&mut self, func: Function) {
        if *func.get_func() == Funcs::Exit {
            return;
        }
        if self.stack.len() != 0 && self.pointer != (self.stack.len() - 1) as i64 {
            while self.pointer + 1 < self.stack.len() as i64 {
                if self.stack.pop().is_none() {
                    break;
                }
            }
        }
        self.stack.push(func);
        self.pointer = (self.stack.len() - 1) as i64;
    }

    pub fn undo(&mut self) -> Option<&Function> {
        if self.pointer == (self.stack.len() - 1) as i64 {
            self.pointer -= 1;
            return self.stack.last();
        } else if self.pointer >= 0 {
            self.pointer -= 1;
            return self.stack.get((self.pointer + 1) as usize);
        } else {
            None
        }
    }

    pub fn redo(&mut self) -> Option<&Function> {
        if self.pointer == (self.stack.len() - 1) as i64 {
            return None;
        } else if self.stack.len() != 0 && self.pointer < (self.stack.len() - 1) as i64 {
            self.pointer += 1;
            self.stack.get(self.pointer as usize)
        } else {
            return None;
        }
    }
}
