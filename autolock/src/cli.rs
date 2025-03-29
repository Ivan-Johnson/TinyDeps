use argh::FromArgs;

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
struct StartDaemonConfig {}

impl StartDaemonConfig {
	pub fn main(self) -> ! {
		todo!();
	}
}

pub fn parse_args() -> MyParsedArgs {
	let args: MyParsedArgs = argh::from_env();
	args
}
