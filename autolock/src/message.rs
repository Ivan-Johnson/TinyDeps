use itj_daemon::DaemonDPK;

#[derive(Debug)]
pub enum AutolockMsg {
	Greet(String),
	SetServerName(String),
}

#[derive(Default)]
pub struct AutolockDPK {
	_server_name: Option<String>,
}

impl DaemonDPK<AutolockMsg> for AutolockDPK {
	fn serialize(msg: &AutolockMsg) -> Vec<u8> {
		let (msg_type, str_val): (u8, &String) = match msg {
			AutolockMsg::Greet(str_val) => (0, str_val),
			AutolockMsg::SetServerName(str_val) => (1, str_val),
		};

		let mut final_msg: Vec<u8> = Vec::new();
		final_msg.push(msg_type);
		final_msg.extend_from_slice(str_val.as_bytes());
		final_msg
	}

	fn deserialize(msg: &[u8]) -> AutolockMsg {
		let msg_type = msg[0];
		let msg_str = std::str::from_utf8(&msg[1..])
			.expect("Could not parse message data: {msg}")
			.to_string();
		match msg_type {
			0 => AutolockMsg::Greet(msg_str),
			1 => AutolockMsg::SetServerName(msg_str),
			2_u8..=u8::MAX => panic!(),
		}
	}

	fn process(msg: &AutolockMsg) {
		println!("Processing {msg:?}");
		match msg {
			AutolockMsg::Greet(name) => println!("Hello {name}, I am {}!", "TODO"),
			AutolockMsg::SetServerName(name) => {
				println!("TODO: Update server name to {name}");
			}
		}
	}
}
