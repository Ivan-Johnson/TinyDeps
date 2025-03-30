use std::cell::RefCell;

use crate::ipc::base::IPC1;
use crate::TcpPort;

// impl base ipc trait using nc

pub struct IPCNC {
	counter: RefCell<u8>,
}

impl IPC1 for IPCNC {
	fn read(&self) -> Vec<u8> {
		let mut counter = self.counter.borrow_mut();
		*counter += 1;

		if *counter > 7 {
			*counter = 0;
		}

		if *counter == 0 {
			vec![0]
		} else if *counter == 3 {
			vec![1]
		} else {
			vec![]
		}
	}

	fn send(&self, msg: &Vec<u8>) {
		// TODO
		println!("Pretending to send message {msg:?}");
	}
}

impl IPCNC {
	pub fn new(_port: TcpPort) -> Self {
		Self { counter: 30.into() }
	}
}
