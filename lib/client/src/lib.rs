use std::{
    io::Write,
    net::{Shutdown, TcpStream},
};
#[derive(Debug)]
pub struct Client {
    stream: TcpStream,
}

impl Client {
    pub fn new(client: TcpStream) -> Self {
        Client { stream: client }
    }
    pub fn read(&self) -> &TcpStream {
        &self.stream
    }
    pub fn write(&mut self, buf: &[u8]) -> std::io::Result<()> {
        self.stream.write(&buf)?;
        self.stream.flush()
    }

    pub fn close(&self) -> std::io::Result<()> {
        let r = self.stream.shutdown(Shutdown::Both);
        r
    }
}

impl Clone for Client {
    fn clone(&self) -> Self {
        let stream = match self.stream.try_clone() {
            | Ok(s) => s,
            | Err(_) => todo!("Si vez esto es un error"),
        };
        Self { stream }
    }
}
