use std::{
    io::Write,
    net::{Shutdown, TcpStream},
};

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
        self.stream.flush()?;
        Ok(())
    }

    pub fn close(&self) -> std::io::Result<()> {
        self.stream.shutdown(Shutdown::Both)?;
        Ok(())
    }
}
