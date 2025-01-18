use std::collections::VecDeque;

use crate::piece_table::PieceTable;

use super::reversable_function::ReversableFunction;

pub struct UndoRedo{
    stack: VecDeque<PieceTable>,
    pointer: i64,
}

impl UndoRedo{
    pub fn new() -> UndoRedo{
        UndoRedo{
            stack: VecDeque::new(),
            pointer: -1,
        }
    }

    pub fn undo(&mut self) -> Option<PieceTable>{
        if self.stack.len() != 0 && self.pointer >= 0{
            self.pointer = self.pointer - 1;
            self.stack.get((self.pointer + 1) as usize).cloned()
        }else{
            None
        }
    }

    pub fn push(&mut self, document: PieceTable){
        if self.pointer > 0{
            while self.pointer + 1 < self.stack.len() as i64{
                self.stack.pop_back();
            }
        }
        self.stack.push_back(document);
        self.pointer = (self.stack.len() - 1) as i64;
    }

    pub fn redo(&mut self) -> Option<PieceTable>{
        if self.stack.len() != 0{
            let doc = self.stack.get(self.pointer as usize).cloned();
            self.pointer = std::cmp::min((self.stack.len() - 1) as i64, self.pointer + 1);
            doc
        }else{
            None
        }
    }
}

pub struct UndoRedo2{
    stack: VecDeque<ReversableFunction>,
    pointer: i64
}

impl UndoRedo2{
    pub fn new() -> UndoRedo2{
        UndoRedo2{
            stack: VecDeque::new(),
            pointer: -1,
        }
    }

    pub fn push(&mut self, func: ReversableFunction){
        if self.pointer > 0{
            while self.pointer + 1 < self.stack.len() as i64{
                self.stack.pop_back();
            }
        }
        self.stack.push_back(func);
        self.pointer = (self.stack.len() - 1) as i64;
    }

    pub fn undo(&mut self) -> Option<ReversableFunction>{
        // if self.stack.len() != 0 && self.pointer >= 0{
        //     self.pointer = self.pointer - 1;
        //     self.stack.get((self.pointer + 1) as usize)
        // }else{
        //     None
        // }
        self.stack.pop_back()
    }

    pub fn redo(&mut self) -> Option<&ReversableFunction>{
        if self.stack.len() != 0{
            let func = self.stack.get(self.pointer as usize);
            self.pointer = std::cmp::min((self.stack.len() - 1) as i64, self.pointer + 1);
            func
        }else{
            None
        }
    }
}