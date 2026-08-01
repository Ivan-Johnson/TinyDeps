use crate::command::run_cmd_output;
use crate::errors::ErrorSmart;
use crate::notification::Notification;

#[derive(Debug, Default, Clone, Copy)]
pub struct NotifySend;

impl NotifySend {
	#[must_use]
	pub const fn new() -> Self {
		Self
	}
}

impl Notification for NotifySend {
	fn send(&self, title: &str, body: &str) -> Result<(), ErrorSmart> {
		run_cmd_output(&["notify-send", title, body]).map(|_| ())
	}
}
