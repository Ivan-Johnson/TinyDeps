use super::child_kill_on_drop::ChildKillOnDrop;
use super::join_handle_join_on_drop::JoinHandleJoinOnDrop;
use crate::errors::ErrorSmart;
use serde::Deserialize;
use std::fmt::Debug;
use std::io::BufRead;
use std::io::BufReader;
use std::process::ChildStdout;
use std::process::Command;
use std::process::Stdio;

pub struct SwayInputSubscribe {
	_swaymsg_child: ChildKillOnDrop,
	_worker: JoinHandleJoinOnDrop<()>,
}

#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
pub struct SwayInputEvent {
	pub change: String,
	pub input: SwayInput,
}

#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
pub struct SwayInput {
	pub identifier: String,
	#[serde(rename = "type")]
	pub kind: String,
}

impl SwayInputSubscribe {
	fn process_input_events<F>(stdout: ChildStdout, mut handler: F)
	where
		F: FnMut(SwayInputEvent),
	{
		for line in BufReader::new(stdout).lines() {
			let line = line.unwrap();
			let event = serde_json::from_str::<SwayInputEvent>(&line).unwrap();
			handler(event);
		}
	}

	pub fn new<F>(handler: F) -> Result<Self, ErrorSmart>
	where
		F: FnMut(SwayInputEvent) + Send + 'static,
	{
		let mut swaymsg_child = Command::new("swaymsg")
			.args(["-m", "-t", "subscribe", "[\"input\"]"])
			.stdout(Stdio::piped())
			.spawn()
			.or_else(|err| {
				ErrorSmart::new_heavy(format!("Failed to launch sway input subscriber: {err}"))
			})?;

		let Some(stdout) = swaymsg_child.stdout.take() else {
			return ErrorSmart::new_heavy("Failed to capture stdout from sway input subscriber".into());
		};

		let worker = std::thread::spawn(move || Self::process_input_events(stdout, handler));

		Ok(Self {
			_swaymsg_child: ChildKillOnDrop::new(swaymsg_child),
			_worker: JoinHandleJoinOnDrop::new(worker),
		})
	}
}

impl Debug for SwayInputSubscribe {
	fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
		fmt.debug_struct("SwayInputSubscribe").finish()
	}
}
