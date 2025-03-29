use itj_daemon::DaemonDPK;

#[derive(Default, Debug)]
pub enum AutolockMsg {
	#[default]
	HelloWorld,
	GoodbyeWorld,
}

#[derive(Default)]
pub struct AutolockDPK {}

impl DaemonDPK<AutolockMsg> for AutolockDPK {
	fn serialize(msg: AutolockMsg) -> Vec<u8> {
		match msg {
			AutolockMsg::HelloWorld => vec![0],
			AutolockMsg::GoodbyeWorld => vec![1],
		}
	}

	fn deserialize(msg: &[u8]) -> AutolockMsg {
		assert!(msg.len() == 1);
		match msg[0] {
			0 => AutolockMsg::HelloWorld,
			1 => AutolockMsg::GoodbyeWorld,
			2_u8..=u8::MAX => panic!(),
		}
	}

	fn process(msg: &AutolockMsg) {
		println!("Processing {msg:?}");
		match msg {
			AutolockMsg::HelloWorld => println!("Hello, World!"),
			AutolockMsg::GoodbyeWorld => println!("Goodbye, World!"),
		}
	}
}
