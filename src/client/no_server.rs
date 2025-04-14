use std::sync::mpsc::{self, Receiver, Sender};

use crate::util::functions::Function;

use super::client_server::{ClientServer, RecieveError};

pub struct NoServer{
    in_channel: Sender<Function>,
    out_channel: Receiver<Function>
}

impl ClientServer for NoServer{
    fn send(&mut self, func: Function) {
        self.in_channel.send(func).unwrap();
    }

    fn recieve(&self) -> Result<Function, RecieveError> {
        match self.out_channel.try_recv(){
            Ok(function) => Ok(function),
            Err(_) => Err(RecieveError::NoData),
        }
    }
}

impl NoServer{
    pub fn run() -> NoServer{
        let (inp, otp) = mpsc::channel::<Function>();
        NoServer{
            in_channel: inp,
            out_channel: otp,
        }
    }
}