pub type TcpPort = u16;

pub trait IPC {
	fn read(&mut self) -> Vec<u8>;
	fn send(&mut self, msg: &Vec<u8>);
	fn restart(&mut self);
}
