use crate::ipc::Connection;
use std::io::Error;
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

	pub fn read_message(&mut self) -> Result<Option<TMsg>, Error> {
		let bytes = self.ipc.read()?;

		if bytes.is_empty() {
			Ok(None)
		} else {
			Ok(Some(TMsg::deserialize(&bytes)))
		}
	}

	pub fn send_message(&mut self, message: &TMsg) {
		let bytes = TMsg::serialize(message);
		self.ipc.send(&bytes).unwrap();
	}
}
