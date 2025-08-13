use std::io::Error;

pub type TcpPort = u16;

pub trait Connection {
	fn read(&mut self) -> Result<Vec<u8>, Error>;
	// TODO: rename to write?
	fn send(&mut self, msg: &[u8]) -> Result<(), Error>;
}

pub trait Server<C: Connection> {
	fn poll_connection(&mut self) -> Option<C>;
}
