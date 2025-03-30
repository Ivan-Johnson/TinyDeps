use argh::FromArgValue;
use itj_daemon::Client;
use itj_daemon::Server;
use std::time::Duration;

use crate::message::AutolockDPK;
use crate::message::AutolockMsg;
use argh::FromArgs;
use itj_daemon::TcpPort;

const DEFAULT_PORT: TcpPort = 15829;

/// TODO document this
#[derive(FromArgs)]
pub struct MyParsedArgs {
	#[argh(subcommand)]
	subcommand: SubcommandCLI,
}

impl MyParsedArgs {
	pub fn main(self) -> ! {
		self.subcommand.main()
	}
}

#[derive(FromArgs)]
#[argh(subcommand)]
enum SubcommandCLI {
	StartDaemon(StartDaemonConfig),
	Demo(DemoConfig),
}

impl SubcommandCLI {
	pub fn main(self) -> ! {
		match self {
			SubcommandCLI::StartDaemon(conf) => conf.main(),
			SubcommandCLI::Demo(conf) => conf.main(),
		}
	}
}

/// TODO document this
#[derive(FromArgs)]
#[argh(subcommand, name = "demo")]
struct DemoConfig {
	/// TODO document this
	#[argh(option, default = "DEFAULT_PORT")]
	port: TcpPort,
	/// TODO document this
	#[argh(positional, default = "DemoMsg::Hello")]
	msg: DemoMsg,
}

// TODO argh v0.1.14 will support this?
// https://github.com/google/argh/commit/79d3022364d7df5f43c4b7e8e1826d50dd04e669
// #[derive(FromArgValue)]
enum DemoMsg {
	// #[argh(name = "hello")]
	Hello,
	_Goodbye,
}

impl DemoMsg {
	pub fn convert(&self) -> AutolockMsg {
		match self {
			DemoMsg::Hello => AutolockMsg::HelloWorld,
			DemoMsg::_Goodbye => AutolockMsg::GoodbyeWorld,
		}
	}
}

impl FromArgValue for DemoMsg {
	fn from_arg_value(arg: &str) -> Result<Self, String> {
		match arg {
			"hello" => Ok(DemoMsg::Hello),
			"goodbye" => Ok(DemoMsg::_Goodbye),
			&_ => Err(format!("\"{arg}\" could not be parsed as a message")),
		}
	}
}

impl DemoConfig {
	pub fn main(self) -> ! {
		let mut client = Client::<AutolockMsg, AutolockDPK>::new(self.port);
		client.send_message(&self.msg.convert());
		std::process::exit(0)
	}
}

/// TODO document this
#[derive(FromArgs)]
#[argh(subcommand, name = "daemon")]
struct StartDaemonConfig {
	/// TODO document this
	#[argh(option, default = "DEFAULT_PORT")]
	port: TcpPort,
}

impl StartDaemonConfig {
	pub fn main(self) -> ! {
		let mut server = Server::<AutolockMsg, AutolockDPK>::new(self.port);

		let mut count = 0;
		loop {
			println!("Poll #{count}");
			server.poll();
			std::thread::sleep(Duration::from_secs(1));
			count += 1;
		}
	}
}

pub fn parse_args() -> MyParsedArgs {
	let args: MyParsedArgs = argh::from_env();
	args
}
