use crate::plugins::Plugin;
use core::cell::RefCell;
use core::fmt::Debug;
use core::time::Duration;
use std::rc::Rc;

impl Default for TallyPlugin {
	fn default() -> Self {
		Self::new(Duration::ZERO)
	}
}

/// A minimal `Plugin` implementation that tracks how many times `poll()` has been called.
#[derive(Debug)]
pub struct TallyPlugin {
	data: Rc<RefCell<usize>>,
	cooldown: Duration,
}

impl TallyPlugin {
	/// Create a fresh tally plugin with `poll_count` = 0.
	pub fn new(cooldown: Duration) -> Self {
		Self {
			data: Rc::new(RefCell::new(0)),
			cooldown,
		}
	}

	/// Return the number of times `poll()` has been called across all clones.
	pub fn get_poll_count(&self) -> usize {
		*self.data.borrow()
	}

	/// Create a shallow clone sharing the same internal data.
	///
	/// Call `get_poll_count()` on the clone to read counters updated
	/// when the original (or any other clone) is polled.
	pub fn shallow_clone(&self) -> Self {
		Self {
			data: self.data.clone(),
			cooldown: self.cooldown,
		}
	}
}

impl Plugin for TallyPlugin {
	fn poll(self: Box<Self>) -> (Box<dyn Plugin>, Duration) {
		*self.data.borrow_mut() += 1;
		let ret_val = self.cooldown;
		(self, ret_val)
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_clone_shares_count() {
		let tally = TallyPlugin::new(Duration::ZERO);
		let clone = tally.shallow_clone();

		assert_eq!(tally.get_poll_count(), 0);
		assert_eq!(clone.get_poll_count(), 0);

		let _ = Box::new(tally).poll();

		assert_eq!(clone.get_poll_count(), 1);
	}
}
