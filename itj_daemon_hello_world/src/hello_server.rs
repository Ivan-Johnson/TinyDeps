use crate::hello_message::HelloWorldMessage;
use itj_tiny_deps::ipc::ipc_linux::InetServer;
use itj_tiny_deps::ipc::MessageConnection;
use itj_tiny_deps::ipc::Server;
use itj_tiny_deps::ipc::TcpPort;
use std::env;
use std::io::ErrorKind;
use std::time::Duration;

pub struct HelloServer {
	server_name: String,
	server: InetServer,
	count: u32,
}

impl HelloServer {
	pub fn new(port: TcpPort) -> Self {
		let server_name = env::var("ITJ_DAEMON_HELLO_WORLD_DEFAULT_SERVER_NAME")
			.unwrap()
			.to_string();
		let server = InetServer::new(port);
		Self {
			server_name,
			count: 0,
			server,
		}
	}

	fn process(&mut self, msg: &HelloWorldMessage) -> HelloWorldMessage {
		match msg {
			HelloWorldMessage::Greet(name) => {
				let response = format!("Hello {name}, I am {}!", self.server_name);
				println!("{}", response);
				HelloWorldMessage::GreetingResponse(response)
			}
			HelloWorldMessage::SetServerName(name) => {
				println!("Changing server name from {} to {}", self.server_name, name);
				self.server_name = name.to_string();
				HelloWorldMessage::Ack
			}
			&HelloWorldMessage::GreetingResponse(_) | &HelloWorldMessage::Ack => {
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
			std::thread::sleep(Duration::from_millis(300));
			self.do_something();

			let connection = self.server.poll_connection();
			if connection.is_none() {
				continue;
			};
			let mut connection = MessageConnection::new(connection.unwrap());
			loop {
				let result = connection.read_message();
				if let Ok(None) = result {
					println!("Connection closed");
					break;
				}
				if let Ok(Some(ref msg)) = result {
					println!("Processing message: {msg:?}");
					let resp = self.process(msg);
					connection.send_message(&resp);
				}
				let err = result.unwrap_err();
				assert_eq!(ErrorKind::WouldBlock, err.kind());
				std::thread::sleep(Duration::from_millis(50));
			}
		}
	}
}
