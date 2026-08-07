use crate::errors::ErrorSmart;
use std::process::Child;
use std::process::Command;
use std::process::ExitStatus;
use std::process::Output;

#[must_use]
pub fn run_cmd_async(args: &[&str]) -> Child {
	assert!(!args.is_empty());
	let mut cmd = Command::new(args[0]);
	cmd.args(&args[1..]);
	match cmd.spawn() {
		Ok(child) => return child,
		Err(err) => panic!("ERROR: could not launch {cmd:?} - {err:?}"),
	}
}

#[must_use]
pub fn run_cmd_async_vec(args: Vec<String>) -> Child {
	// yuck.
	let args: Vec<&str> = args.iter().map(|string| &**string).collect();
	run_cmd_async(&args)
}

pub fn wait_for_child(mut child: Child) {
	let status: ExitStatus = child.wait().unwrap();
	assert!(status.success());
}

pub fn run_cmd_output(args: &[&str]) -> Result<Output, ErrorSmart> {
	assert!(!args.is_empty());
	let output = Command::new(args[0])
		.args(&args[1..])
		.output()
		.or_else(|err| ErrorSmart::new_heavy(format!("Failed to run command {args:?}: {err}")))?;

	if output.status.success() {
		return Ok(output);
	}

	let stderr = String::from_utf8_lossy(&output.stderr);
	ErrorSmart::new_heavy(format!(
		"Command {:?} failed with status {:?}: {}",
		args,
		output.status.code(),
		stderr.trim(),
	))
}

pub fn run_cmd_sync(args: &[&str]) {
	let child = run_cmd_async(args);
	wait_for_child(child);
}
