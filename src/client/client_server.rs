use crate::util::functions::Function;

pub trait ClientServer{
    fn send(&mut self, func: Function);

    fn recieve(&self) -> Result<Function, RecieveError>;
}

pub enum RecieveError{
    NoData,
    ConnectionClosed,
}