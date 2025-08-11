use crate::ipc::Connection;
use std::io::ErrorKind;
use std::marker::PhantomData;

pub trait Message {
	fn serialize(&self) -> Vec<u8>;
	fn deserialize(msg: &[u8]) -> Self;
}

pub struct MessageConnection<TMsg: Message, TConnection: Connection> {
	ipc: TConnection,
	_phantom_msg: PhantomData<TMsg>,
}

impl<TMsg: Message, TConnection: Connection> MessageConnection<TMsg, TConnection> {
	pub fn new(connection: TConnection) -> Self {
		Self {
			ipc: connection,
			_phantom_msg: PhantomData {},
		}
	}

	pub fn read_message(&mut self) -> Result<TMsg, ErrorKind> {
		let bytes = self.ipc.read()?;
		Ok(TMsg::deserialize(&bytes))
	}

	pub fn send_message(&mut self, message: &TMsg) {
		let bytes = TMsg::serialize(message);
		self.ipc.send(&bytes).unwrap();
	}
}
