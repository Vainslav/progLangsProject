use std::{io::{ErrorKind, Error}, net::TcpStream};

use tungstenite::{accept, Message, WebSocket};

use crate::util::functions::Function;

use super::client_server::ClientServer;

pub struct LocalServer{
    WS: WebSocket<TcpStream>,
}

impl ClientServer for LocalServer {
    fn send(&mut self, func: Function) {
        let _ = self.WS.send(Message::Binary(tungstenite::Bytes::from_iter(bincode::serialize(&func).unwrap())));
    }
    
    fn recieve(&self) -> Result<Function, super::client_server::RecieveError> {
        
    }
}

impl LocalServer {
    pub fn run(connect_to: String) -> Result<LocalServer, Error> {
        let tcp_stream = TcpStream::connect(connect_to)?;

        let websocket = match accept(tcp_stream){
            Ok(ws) => Ok(ws),
            Err(_) => Err(Error::new(ErrorKind::NotConnected, "failed connecting to ws")),
        }?;

        Ok(LocalServer { WS: websocket })
    }
}