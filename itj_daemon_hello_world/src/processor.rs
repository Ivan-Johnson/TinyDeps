use crate::message::AutolockMsg;
use itj_tiny_deps::daemon::MessageProcessor;

pub struct Processor {
	server_name: String,
}

impl MessageProcessor<AutolockMsg> for Processor {
	fn process(&mut self, msg: &AutolockMsg) -> AutolockMsg {
		match msg {
			AutolockMsg::Greet(name) => {
				let response = format!("Hello {name}, I am {}!", self.server_name);
				println!("{}", response);
				AutolockMsg::GreetingResponse(response)
			}
			AutolockMsg::SetServerName(name) => {
				println!("Changing server name from {} to {}", self.server_name, name);
				self.server_name = name.to_string();
				AutolockMsg::Ack
			}
			&AutolockMsg::GreetingResponse(_) | &AutolockMsg::Ack => {
				panic!("A response was sent as a request??")
			}
		}
	}
}

impl Default for Processor {
	fn default() -> Self {
		Self {
			server_name: "Alice".to_string(),
		}
	}
}
