use crate::daemon::MessageSerializer;
use crate::ipc::TcpPort;
use crate::ipc::IPC;
use crate::ipc::IPCNC;
use std::fmt::Debug;
use std::marker::Send;
use std::sync::mpsc;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;
use std::thread;
use std::time::Duration;

fn server_thread_main<TMsg: Debug + Send + 'static, TSerializer: MessageSerializer<TMsg>>(
	port: TcpPort,
	tx: Sender<(TMsg, Sender<TMsg>)>,
) -> ! {
	let mut ipc = IPCNC::open_server(port);
	loop {
		// 1. Read message from Client
		let bytes = ipc.read();
		if bytes.len() == 0 {
			std::thread::sleep(Duration::from_secs(1));
			continue;
		}
		let msg: TMsg = TSerializer::deserialize(&bytes);

		// 2. Send message to main server thread
		let (tx_resp, rx_resp) = mpsc::channel::<TMsg>();
		tx.send((msg, tx_resp)).unwrap();

		// 3. Read response from main server thread
		let response = rx_resp.recv().unwrap();

		// 4. Send response to client
		let response_bytes = TSerializer::serialize(&response);
		ipc.send(&response_bytes);

		ipc.restart();
	}
}

pub fn spawn_server_thread<TMsg: Debug + Send + 'static, TSerializer: MessageSerializer<TMsg>>(
	port: TcpPort,
) -> (thread::JoinHandle<()>, Receiver<(TMsg, Sender<TMsg>)>) {
	let (tx, rx) = mpsc::channel();

	let handle = thread::spawn(move || {
		server_thread_main::<TMsg, TSerializer>(port, tx);
	});

	(handle, rx)
}
