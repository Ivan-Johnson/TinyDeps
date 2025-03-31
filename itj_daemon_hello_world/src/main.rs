#![forbid(unsafe_code)]
use itj_daemon_hello_world::cli::parse_args;

pub fn main() {
	let args = parse_args();
	args.main();
}
