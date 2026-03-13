use crate::errors::ErrorSmart;
use crate::notification::Notification;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

#[derive(Debug, Clone, PartialEq)]
struct MockNotificationCall {
	/// `None` represents "don't care".
	pub title: Option<String>,
	/// `None` represents "don't care".
	pub body: Option<String>,
	pub should_ret_err: bool,
}

#[derive(Debug, Default)]
struct MockNotificationData {
	calls: VecDeque<MockNotificationCall>,
}

#[derive(Debug, Default, Clone)]
pub struct MockNotification {
	data: Rc<RefCell<MockNotificationData>>,
}

impl MockNotification {
	pub fn new() -> Self {
		Self::default()
	}

	pub fn flush(&self) {
		let mut data = self.data.borrow_mut();
		if !data.calls.is_empty() {
			// If the `flush` panics as part of the test, we don't want to panic a second time as part of the `drop`
			let vec: Vec<_> = data.calls.drain(0..).collect();
			panic!("{} notification(s) were never sent: {:?}", vec.len(), vec);
		}
	}

	fn expect(&mut self, data: MockNotificationCall) {
		self.data.borrow_mut().calls.push_back(data)
	}

	pub fn expect_exact(&mut self, title: String, body: String) {
		self.expect(MockNotificationCall {
			title: Some(title),
			body: Some(body),
			should_ret_err: false,
		})
	}

	pub fn expect_any(&mut self) {
		self.expect(MockNotificationCall {
			title: None,
			body: None,
			should_ret_err: false,
		})
	}

	pub fn expect_any_ret_err(&mut self) {
		self.expect(MockNotificationCall {
			title: None,
			body: None,
			should_ret_err: true,
		})
	}
}

impl Drop for MockNotification {
	fn drop(&mut self) {
		self.flush();
	}
}

impl Notification for MockNotification {
	fn send(&self, title: &str, body: &str) -> Result<(), ErrorSmart> {
		let data: &mut MockNotificationData = &mut self.data.borrow_mut();
		println!("NOTIFICATION: {:?}, {:?}", title, body);
		let expected_val = data
			.calls
			.pop_front()
			.expect("^ This notification was unexpected");
		if let Some(expected) = expected_val.title {
			assert_eq!(expected, title);
		}
		if let Some(expected) = expected_val.body {
			assert_eq!(expected, body);
		}
		if expected_val.should_ret_err {
			ErrorSmart::new_light("Mock Error")
		} else {
			Ok(())
		}
	}
}

#[cfg(test)]
mod tests {
	#![allow(clippy::large_stack_frames)]

	use super::*;
	use crate::notification::Notification;

	const TITLE1: &str = "Test Title";
	const BODY1: &str = "Test Body";
	const TITLE2: &str = "Another Title";
	const BODY2: &str = "Another Body";

	#[test]
	fn test_expect_exact_happy() {
		let mut mn = MockNotification::new();
		mn.expect(MockNotificationCall {
			title: Some(TITLE1.to_string()),
			body: Some(BODY1.to_string()),
			should_ret_err: false,
		});
		mn.send(TITLE1, BODY1).unwrap();
	}

	#[test]
	fn test_expect_any_happy() {
		let mut mn = MockNotification::new();
		mn.expect_any();
		mn.send(TITLE1, BODY1).unwrap();
	}

	#[test]
	fn test_two_expects_consumed_lifo() {
		let mut mn = MockNotification::new();
		mn.expect_exact(TITLE1.to_string(), BODY1.to_string());
		mn.expect_exact(TITLE2.to_string(), BODY2.to_string());
		mn.send(TITLE1, BODY1).unwrap();
		mn.send(TITLE2, BODY2).unwrap();
	}

	#[test]
	#[should_panic]
	fn test_flush_panic() {
		let mut mn = MockNotification::new();
		mn.expect_exact(TITLE1.to_string(), BODY1.to_string());
		mn.flush();
	}

	#[test]
	#[should_panic]
	fn test_drop_panic() {
		let mut mn = MockNotification::new();
		mn.expect_any();
	}

	#[test]
	#[should_panic]
	fn test_wrong_title_panics() {
		let mut mn = MockNotification::new();
		mn.expect(MockNotificationCall {
			title: Some(TITLE1.to_string()),
			body: None,
			should_ret_err: false,
		});
		let _ = mn.send("wrong title", BODY1);
	}

	#[test]
	#[should_panic]
	fn test_wrong_body_panics() {
		let mut mn = MockNotification::new();
		mn.expect(MockNotificationCall {
			title: None,
			body: Some(BODY1.to_string()),
			should_ret_err: false,
		});
		let _ = mn.send(TITLE1, "wrong body");
	}

	#[test]
	fn test_expect_any_ret_err_returns_err() {
		let mut mn = MockNotification::new();
		mn.expect_any_ret_err();
		let result = mn.send(TITLE1, BODY1);
		assert!(result.is_err());
	}

	#[test]
	fn test_expect_any_returns_ok() {
		let mut mn = MockNotification::new();
		mn.expect_any();
		let result = mn.send(TITLE1, BODY1);
		assert!(result.is_ok());
	}

	#[test]
	#[should_panic(expected = "This notification was unexpected")]
	fn test_unexpected_send_panics() {
		let mn = MockNotification::new();
		let _ = mn.send(TITLE1, BODY1);
	}

	#[test]
	fn test_clone_shares_state() {
		let mut original = MockNotification::new();
		original.expect_any();
		let clone = original.clone();
		clone.send(TITLE1, BODY1).unwrap();
	}
}
