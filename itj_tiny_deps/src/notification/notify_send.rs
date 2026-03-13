use crate::errors::ErrorSmart;
use crate::notification::Notification;
use std::process::Command;

#[derive(Debug, Default, Clone, Copy)]
pub struct NotifySend;

impl NotifySend {
	pub fn new() -> Self {
		Self
	}
}

impl Notification for NotifySend {
	fn send(&self, title: &str, body: &str) -> Result<(), ErrorSmart> {
		let mut cmd = Command::new("notify-send");
		cmd.arg(title).arg(body);

		let output = match cmd.output() {
			Ok(output) => output,
			Err(err) => {
				return ErrorSmart::new_heavy(format!(
					"Failed to launch `notify-send` for title {title:?}: {err}"
				));
			}
		};

		if output.status.success() {
			return Ok(());
		}

		let stderr = String::from_utf8_lossy(&output.stderr);
		ErrorSmart::new_heavy(format!(
			"`notify-send` exited with status {:?} for title {title:?}: {}",
			output.status.code(),
			stderr.trim()
		))
	}
}
