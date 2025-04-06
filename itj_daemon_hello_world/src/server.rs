use crate::message::DaemonDemoMsg;
use itj_tiny_deps::daemon::spawn_server_thread;
use itj_tiny_deps::ipc::TcpPort;
use std::env;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;
use std::sync::mpsc::TryRecvError;
use std::thread;
use std::time::Duration;

pub struct Server {
	server_name: String,
	receiver: Receiver<(DaemonDemoMsg, Sender<DaemonDemoMsg>)>,
	tiny_server_handle: thread::JoinHandle<()>,
	count: u32,
}

impl Server {
	pub fn new(port: TcpPort) -> Self {
		let server_name = env::var("ITJ_DAEMON_HELLO_WORLD_DEFAULT_SERVER_NAME")
			.unwrap()
			.to_string();
		let (tiny_server_handle, receiver) = spawn_server_thread::<DaemonDemoMsg, DaemonDemoMsg>(port);
		let count = 0;
		Self {
			tiny_server_handle,
			server_name,
			receiver,
			count,
		}
	}

	fn process(&mut self, msg: &DaemonDemoMsg) -> DaemonDemoMsg {
		match msg {
			DaemonDemoMsg::Greet(name) => {
				let response = format!("Hello {name}, I am {}!", self.server_name);
				println!("{}", response);
				DaemonDemoMsg::GreetingResponse(response)
			}
			DaemonDemoMsg::SetServerName(name) => {
				println!("Changing server name from {} to {}", self.server_name, name);
				self.server_name = name.to_string();
				DaemonDemoMsg::Ack
			}
			&DaemonDemoMsg::GreetingResponse(_) | &DaemonDemoMsg::Ack => {
				panic!("A response was sent as a request??")
			}
		}
	}

	fn do_something(&mut self) {
		self.count += 1;
		println!("Poll #{}", self.count);
	}

	pub fn main(mut self) -> ! {
		loop {
			std::thread::sleep(Duration::from_secs(1));
			self.do_something();

			assert!(!self.tiny_server_handle.is_finished());

			// Note: `try_recv` is used so that `do_something` is
			// not blocked. If you don't actually need to do
			// anything other than respond to messages, try using
			// `recv` instead.
			let new_msg = self.receiver.try_recv();
			if let Err(TryRecvError::Empty) = new_msg {
				continue;
			};
			let (msg, tx_resp) = new_msg.unwrap();
			let resp = self.process(&msg);
			tx_resp.send(resp).unwrap();
		}
	}
}
