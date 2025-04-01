use crate::message::AutolockMsg;
use itj_tiny_deps::daemon::MessageProcessor;

pub struct Processor {
	server_name: String,
}

impl MessageProcessor<AutolockMsg> for Processor {
	fn process(&mut self, msg: &AutolockMsg) -> Result<Option<AutolockMsg>, ()> {
		match msg {
			AutolockMsg::Greet(name) => {
				println!("Hello {name}, I am {}!", self.server_name);
				// TODO send a response
			}
			AutolockMsg::SetServerName(name) => {
				println!("Changing server name from {} to {}", self.server_name, name);
				self.server_name = name.to_string();
				// TODO send a response
			}
		};
		Ok(None)
	}
}

impl Default for Processor {
	fn default() -> Self {
		Self {
			server_name: "Alice".to_string(),
		}
	}
}
