use crate::message::DaemonDemoMsg;
use itj_tiny_deps::daemon::MessageProcessor;
use std::env;

pub struct Processor {
	server_name: String,
}

impl MessageProcessor<DaemonDemoMsg> for Processor {
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
}

impl Default for Processor {
	fn default() -> Self {
		let server_name = env::var("ITJ_DAEMON_HELLO_WORLD_DEFAULT_SERVER_NAME")
			.unwrap()
			.to_string();
		Self { server_name }
	}
}
