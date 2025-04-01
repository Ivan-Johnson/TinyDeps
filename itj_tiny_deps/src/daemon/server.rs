use crate::daemon::MessageProcessor;
use crate::daemon::MessageSerializer;
use crate::ipc::TcpPort;
use crate::ipc::IPC;
use crate::ipc::IPCNC;
use std::fmt::Debug;
use std::marker::PhantomData;

pub struct Server<TMsg: Debug, TSerializer: MessageSerializer<TMsg>, TProcessor: MessageProcessor<TMsg>> {
	ipc: Box<dyn IPC>,
	processor: TProcessor,
	_phantom_tmsg: PhantomData<TMsg>,
	_phantom_serializer: PhantomData<TSerializer>,
}

impl<TMsg: Debug, TSerializer: MessageSerializer<TMsg>, TProcessor: MessageProcessor<TMsg>>
	Server<TMsg, TSerializer, TProcessor>
{
	pub fn new(port: TcpPort, processor: TProcessor) -> Self {
		Self {
			ipc: Box::new(IPCNC::open_server(port)),
			processor,
			_phantom_tmsg: PhantomData {},
			_phantom_serializer: PhantomData {},
		}
	}

	pub fn poll(&mut self) {
		loop {
			let bytes = self.ipc.read();
			if bytes.len() == 0 {
				break;
			}
			let msg: TMsg = TSerializer::deserialize(&bytes);
			let response = self.processor.process(&msg);
			let response_bytes = TSerializer::serialize(&response);
			self.ipc.send(&response_bytes);

			self.ipc.restart();
		}
	}
}
