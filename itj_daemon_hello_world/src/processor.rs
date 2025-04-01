use crate::message::AutolockMsg;
use itj_tiny_deps::daemon::MessageProcessor;

#[derive(Default)]
pub struct Processor {
	_server_name: Option<String>,
}

impl MessageProcessor<AutolockMsg> for Processor {
	fn process(&mut self, msg: &AutolockMsg) -> std::option::Option<AutolockMsg> {
		println!("Processing {msg:?}");
		match msg {
			AutolockMsg::Greet(name) => println!("Hello {name}, I am {}!", "TODO"),
			AutolockMsg::SetServerName(name) => {
				println!("TODO: Update server name to {name}");
			}
		};
		None
	}
}
