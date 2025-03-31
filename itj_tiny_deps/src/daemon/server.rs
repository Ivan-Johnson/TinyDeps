use crate::daemon::message::DaemonDPK;
use crate::ipc::TcpPort;
use crate::ipc::IPC;
use crate::ipc::IPCNC;
use std::marker::PhantomData;

pub struct Server<TMsg, TDPK: DaemonDPK<TMsg>> {
	ipc: Box<dyn IPC>,
	_phantom_tdpk: PhantomData<TDPK>,
	_phantom_tmsg: PhantomData<TMsg>,
}

impl<TMsg, TDPK: DaemonDPK<TMsg>> Server<TMsg, TDPK> {
	pub fn new(port: TcpPort) -> Self {
		Self {
			ipc: Box::new(IPCNC::open_server(port)),
			_phantom_tdpk: PhantomData {},
			_phantom_tmsg: PhantomData {},
		}
	}

	pub fn poll(&mut self) {
		loop {
			let bytes = self.ipc.read();
			if bytes.len() == 0 {
				break;
			}
			let msg: TMsg = TDPK::deserialize(&bytes);
			TDPK::process(&msg);
			self.ipc.restart();
		}
	}
}
