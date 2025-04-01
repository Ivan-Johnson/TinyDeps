use itj_tiny_deps::daemon::MessageSerializer;

// TODO rename this struct
#[derive(Debug, PartialEq)]
pub enum DaemonDemoMsg {
	Greet(String),
	SetServerName(String),
	GreetingResponse(String),
	Ack,
}

impl MessageSerializer<DaemonDemoMsg> for DaemonDemoMsg {
	fn serialize(msg: &DaemonDemoMsg) -> Vec<u8> {
		let (msg_type, str_val): (u8, String) = match msg {
			DaemonDemoMsg::Greet(str_val) => (0, str_val.clone()),
			DaemonDemoMsg::SetServerName(str_val) => (1, str_val.clone()),
			DaemonDemoMsg::GreetingResponse(str_val) => (2, str_val.clone()),
			DaemonDemoMsg::Ack => (3, "".to_string()),
		};

		let mut final_msg: Vec<u8> = Vec::new();
		final_msg.push(msg_type);
		final_msg.extend_from_slice(str_val.as_bytes());
		final_msg
	}

	fn deserialize(msg: &[u8]) -> DaemonDemoMsg {
		let msg_type = msg[0];
		let msg_str = std::str::from_utf8(&msg[1..])
			.expect("Could not parse message data: {msg}")
			.to_string();
		match msg_type {
			0 => DaemonDemoMsg::Greet(msg_str),
			1 => DaemonDemoMsg::SetServerName(msg_str),
			2 => DaemonDemoMsg::GreetingResponse(msg_str),
			3 => DaemonDemoMsg::Ack,
			4_u8..=u8::MAX => panic!(),
		}
	}
}
