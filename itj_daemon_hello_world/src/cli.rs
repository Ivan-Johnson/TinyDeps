use crate::message::AutolockDPK;
use crate::message::AutolockMsg;
use argh::FromArgs;
use itj_tiny_deps::Client;
use itj_tiny_deps::Server;
use itj_tiny_deps::TcpPort;
use std::time::Duration;

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
	SetServerName(SetServerNameConfig),
	Greet(GreetConfig),
}

impl SubcommandCLI {
	pub fn main(self) -> ! {
		match self {
			SubcommandCLI::StartDaemon(conf) => conf.main(),
			SubcommandCLI::Greet(conf) => conf.main(),
			SubcommandCLI::SetServerName(conf) => conf.main(),
		}
	}
}

/// TODO document this
#[derive(FromArgs)]
#[argh(subcommand, name = "greet")]
struct GreetConfig {
	/// TODO document this
	#[argh(option, default = "DEFAULT_PORT")]
	port: TcpPort,
	/// TODO document this
	#[argh(positional, default = "\"Bob\".to_string()")]
	name: String,
}

impl GreetConfig {
	pub fn main(self) -> ! {
		let msg = AutolockMsg::Greet(self.name);

		let mut client = Client::<AutolockMsg, AutolockDPK>::new(self.port);
		client.send_message(&msg);
		std::process::exit(0)
	}
}

/// TODO document this
#[derive(FromArgs)]
#[argh(subcommand, name = "set-server-name")]
struct SetServerNameConfig {
	/// TODO document this
	#[argh(option, default = "DEFAULT_PORT")]
	port: TcpPort,
	/// TODO document this
	#[argh(positional, default = "\"Alice\".to_string()")]
	new_name: String,
}

impl SetServerNameConfig {
	pub fn main(self) -> ! {
		let msg = AutolockMsg::SetServerName(self.new_name);

		let mut client = Client::<AutolockMsg, AutolockDPK>::new(self.port);
		client.send_message(&msg);
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
