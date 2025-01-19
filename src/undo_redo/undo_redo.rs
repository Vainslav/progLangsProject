use super::reversable_function::ReversableFunction;

pub struct UndoRedo{
    stack: Vec<ReversableFunction>,
    pointer: i64
}

impl UndoRedo{
    pub fn new() -> UndoRedo{
        UndoRedo{
            stack: Vec::new(),
            pointer: -1,
        }
    }

    pub fn push(&mut self, func: ReversableFunction){
        if self.pointer > 0{
            while self.pointer + 1 < self.stack.len() as i64{
                self.stack.pop();
            }
        }
        self.stack.push(func);
        self.pointer = (self.stack.len() - 1) as i64;
    }

    pub fn undo(&mut self) -> Option<ReversableFunction>{
        // if self.stack.len() != 0 && self.pointer >= 0{
        //     self.pointer = self.pointer - 1;
        //     self.stack.get((self.pointer + 1) as usize)
        // }else{
        //     None
        // }
        self.stack.pop()
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