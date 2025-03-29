#![forbid(unsafe_code)]
use autolock::cli::parse_args;

pub fn main() {
	let args = parse_args();
	args.main();
}
