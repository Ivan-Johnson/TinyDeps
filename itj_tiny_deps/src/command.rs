use std::process::Child;
use std::process::Command;
use std::process::ExitStatus;

pub fn run_cmd_async(args: &[&str]) -> Child {
	assert!(args.len() >= 1);
	let cmd = args[0];
	let mut cmd = Command::new(cmd);
	for arg in &args[1..] {
		cmd.arg(arg);
	}
	cmd.spawn().expect("ERROR: could not launch {cmd}")
}

pub fn run_cmd_async_vec(args: Vec<String>) -> Child {
	// yuck.
	let args: Vec<&str> = args.iter().map(|string| &**string).collect();
	run_cmd_async(&args)
}

pub fn wait_for_child(mut child: Child) -> () {
	let status: ExitStatus = child.wait().unwrap();
	assert!(status.success());
}

pub fn run_cmd_sync(args: &[&str]) -> () {
	let child = run_cmd_async(args);
	wait_for_child(child);
}
