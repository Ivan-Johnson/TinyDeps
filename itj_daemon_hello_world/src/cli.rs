use crate::message::DaemonDemoMsg;
use crate::server::Server;
use argh::FromArgs;
use itj_tiny_deps::daemon::Client;
use itj_tiny_deps::ipc::TcpPort;

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
	#[argh(positional, default = "\"Client\".to_string()")]
	name: String,
}

impl GreetConfig {
	pub fn main(self) -> ! {
		let msg = DaemonDemoMsg::Greet(self.name);

		let mut client = Client::<DaemonDemoMsg, DaemonDemoMsg>::new(self.port);
		let response = client.send_message(&msg);
		println!("Got this response: {response:?}");
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
		let msg = DaemonDemoMsg::SetServerName(self.new_name);

		let mut client = Client::<DaemonDemoMsg, DaemonDemoMsg>::new(self.port);
		let response = client.send_message(&msg);
		assert_eq!(response, DaemonDemoMsg::Ack);
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
		let server = Server::new(self.port);
		server.main()
	}
}

pub fn parse_args() -> MyParsedArgs {
	let args: MyParsedArgs = argh::from_env();
	args
}
