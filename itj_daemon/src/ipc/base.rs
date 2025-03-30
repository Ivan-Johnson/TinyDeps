pub type TcpPort = u16;

pub trait IPC1 {
	fn read(&self) -> Vec<u8>;
	fn send(&self, msg: &Vec<u8>);
}
