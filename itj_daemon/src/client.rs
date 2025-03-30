use crate::ipc::base::IPC1;
use crate::ipc::base_impl::nc::IPCNC;
use crate::message::DaemonDPK;
use std::marker::PhantomData;

#[derive(Default)]
pub struct ClientBuilder<TMsg, TDPK: DaemonDPK<TMsg>> {
	_phantom_tdpk: PhantomData<TDPK>,
	_phantom_tmsg: PhantomData<TMsg>,
}

impl<TMsg, TDPK: DaemonDPK<TMsg>> ClientBuilder<TMsg, TDPK> {
	pub fn build(&self) -> Client<TMsg, TDPK> {
		Client {
			ipc: Box::new(IPCNC::open_client(123)),
			_phantom_tdpk: self._phantom_tdpk,
			_phantom_tmsg: self._phantom_tmsg,
		}
	}
}

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
}
