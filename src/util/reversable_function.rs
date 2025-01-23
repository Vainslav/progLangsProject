#[derive(PartialEq)]
pub enum Funcs{
    Insert,
    Remove,
    Delete,
}

pub struct ReversableFunction{
    pub func: Funcs,
    pub index: usize,
    pub string: String,
}

impl ReversableFunction {
    pub fn new(fun: Funcs, idx: usize, str: String) -> ReversableFunction{
        ReversableFunction{
            func: fun,
            index: idx,
            string: str
        }
    }
}