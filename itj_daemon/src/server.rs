use crate::ipc::base::IPC1;
use crate::ipc::base_impl::nc::IPCNC;
use crate::message::DaemonDPK;
use std::marker::PhantomData;

#[derive(Default)]
pub struct ServerBuilder<TMsg, TDPK: DaemonDPK<TMsg>> {
	_phantom_tdpk: PhantomData<TDPK>,
	_phantom_tmsg: PhantomData<TMsg>,
}

impl<TMsg, TDPK: DaemonDPK<TMsg>> ServerBuilder<TMsg, TDPK> {
	pub fn build(&self) -> Server<TMsg, TDPK> {
		Server {
			ipc: Box::new(IPCNC::new(123)),
			_phantom_tdpk: self._phantom_tdpk,
			_phantom_tmsg: self._phantom_tmsg,
		}
	}
}

pub struct Server<TMsg, TDPK: DaemonDPK<TMsg>> {
	ipc: Box<dyn IPC1>,
	_phantom_tdpk: PhantomData<TDPK>,
	_phantom_tmsg: PhantomData<TMsg>,
}

impl<TMsg, TDPK: DaemonDPK<TMsg>> Server<TMsg, TDPK> {
	pub fn poll(&mut self) {
		loop {
			let bytes = self.ipc.read();
			if bytes.len() == 0 {
				break;
			}
			let msg: TMsg = TDPK::deserialize(&bytes);
			TDPK::process(&msg);
		}
	}
}
