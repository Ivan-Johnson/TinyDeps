use crate::hello_message::HelloWorldMessage;
use crate::hello_server::HelloServer;
use argh::FromArgs;
use itj_tiny_deps::ipc::ipc_linux::new_inet_client;
use itj_tiny_deps::ipc::ipc_linux::FDConnection;
use itj_tiny_deps::ipc::MessageConnection;
use itj_tiny_deps::ipc::TcpPort;

// TODO: cleanup. I'm not really sure what.
// * split into separate functions or something?
// * Move to hello_message.rs?
// * actually. Best option is to create a `HelloClient` struct that has pretty APIs?
//   * e.g. `fn greet() -> greetResponse`
fn send_and_get_response(port: TcpPort, msg: &HelloWorldMessage) -> HelloWorldMessage {
	let connection = new_inet_client(port);
	let mut connection = MessageConnection::<HelloWorldMessage, FDConnection>::new(connection);

	connection.send_message(msg);
	let response = connection
		.read_message()
		.expect("Got an error when reading a response");
	assert_ne!(None, response, "Expected a response, but got EOF");
	response.unwrap()
}

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
		let msg = HelloWorldMessage::Greet(self.name);
		let response = send_and_get_response(self.port, &msg);
		let HelloWorldMessage::GreetingResponse(response) = response else {
			panic!("Expected GreetingResponse; got {response:?}");
		};
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
		let msg = HelloWorldMessage::SetServerName(self.new_name);
		let response = send_and_get_response(self.port, &msg);
		assert_eq!(response, HelloWorldMessage::Ack);
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
		let server = HelloServer::new(self.port);
		server.main()
	}
}

pub fn parse_args() -> MyParsedArgs {
	let args: MyParsedArgs = argh::from_env();
	args
}
