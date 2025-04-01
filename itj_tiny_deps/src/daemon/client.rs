use crate::daemon::MessageSerializer;
use crate::ipc::TcpPort;
use crate::ipc::IPC;
use crate::ipc::IPCNC;
use std::marker::PhantomData;

pub struct Client<TMsg, TSerializer: MessageSerializer<TMsg>> {
	ipc: Box<dyn IPC>,
	_phantom_tdpk: PhantomData<TSerializer>,
	_phantom_tmsg: PhantomData<TMsg>,
}

impl<TMsg, TSerializer: MessageSerializer<TMsg>> Client<TMsg, TSerializer> {
	pub fn send_message(&mut self, message: &TMsg) -> TMsg {
		let bytes = TSerializer::serialize(message);
		self.ipc.send(&bytes);
		let response_bytes = self.ipc.read();
		TSerializer::deserialize(&response_bytes)
	}

	pub fn new(server_port: TcpPort) -> Self {
		Self {
			ipc: Box::new(IPCNC::open_client(server_port)),
			_phantom_tdpk: PhantomData {},
			_phantom_tmsg: PhantomData {},
		}
	}
}
