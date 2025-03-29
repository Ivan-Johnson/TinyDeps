use std::time::Duration;

use crate::message::AutolockDPK;
use crate::message::AutolockMsg;
use argh::FromArgs;
use itj_daemon::ServerBuilder;
use itj_daemon::TcpPort;

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
struct DemoConfig {}

impl DemoConfig {
	pub fn main(self) -> ! {
		todo!();
	}
}

/// TODO document this
#[derive(FromArgs)]
#[argh(subcommand, name = "daemon")]
struct StartDaemonConfig {
	/// TODO document this
	#[argh(option, default = "15829")]
	_annouce_port: TcpPort,
}

impl StartDaemonConfig {
	pub fn main(self) -> ! {
		let builder: ServerBuilder<AutolockMsg, AutolockDPK> = ServerBuilder::default();
		let mut server = builder.build();

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
