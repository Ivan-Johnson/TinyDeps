use crate::errors::ErrorSmart;

pub trait Notification {
	fn send(&self, title: &str, body: &str) -> Result<(), ErrorSmart>;
}
