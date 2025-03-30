use crate::ipc::base::IPC1;
use crate::ipc::base_impl::nc::IPCNC;
use crate::message::DaemonDPK;
use crate::TcpPort;
use std::marker::PhantomData;

pub struct Client<TMsg, TDPK: DaemonDPK<TMsg>> {
	ipc: Box<dyn IPC1>,
	_phantom_tdpk: PhantomData<TDPK>,
	_phantom_tmsg: PhantomData<TMsg>,
}

impl<TMsg, TDPK: DaemonDPK<TMsg>> Client<TMsg, TDPK> {
	pub fn send_message(&mut self, message: &TMsg) {
		let bytes = TDPK::serialize(message);
		self.ipc.send(&bytes);
	}

	pub fn new(port: TcpPort) -> Self {
		Self {
			ipc: Box::new(IPCNC::open_client(port)),
			_phantom_tdpk: PhantomData {},
			_phantom_tmsg: PhantomData {},
		}
	}
}
