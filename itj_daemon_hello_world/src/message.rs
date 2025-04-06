use itj_tiny_deps::daemon::MessageSerializer;

#[derive(Debug, PartialEq)]
pub enum HelloWorldMessage {
	Greet(String),
	SetServerName(String),
	GreetingResponse(String),
	Ack,
}

impl MessageSerializer<HelloWorldMessage> for HelloWorldMessage {
	fn serialize(msg: &HelloWorldMessage) -> Vec<u8> {
		let (msg_type, str_val): (u8, String) = match msg {
			HelloWorldMessage::Greet(str_val) => (0, str_val.clone()),
			HelloWorldMessage::SetServerName(str_val) => (1, str_val.clone()),
			HelloWorldMessage::GreetingResponse(str_val) => (2, str_val.clone()),
			HelloWorldMessage::Ack => (3, "".to_string()),
		};

		let mut final_msg: Vec<u8> = Vec::new();
		final_msg.push(msg_type);
		final_msg.extend_from_slice(str_val.as_bytes());
		final_msg
	}

	fn deserialize(msg: &[u8]) -> HelloWorldMessage {
		let msg_type = msg[0];
		let msg_str = std::str::from_utf8(&msg[1..])
			.expect("Could not parse message data: {msg}")
			.to_string();
		match msg_type {
			0 => HelloWorldMessage::Greet(msg_str),
			1 => HelloWorldMessage::SetServerName(msg_str),
			2 => HelloWorldMessage::GreetingResponse(msg_str),
			3 => HelloWorldMessage::Ack,
			4_u8..=u8::MAX => panic!(),
		}
	}
}
