use crate::plugins::Plugin;
use core::cell::RefCell;
use core::fmt::Debug;
use core::time::Duration;
use std::rc::Rc;

/// A minimal `Plugin` implementation that tracks how many times `poll()` has been called.
#[derive(Debug, Default)]
pub struct TallyPlugin {
	data: Rc<RefCell<usize>>,
	cooldown: Duration,
}

impl TallyPlugin {
	pub fn new(cooldown: Duration) -> Self {
		Self {
			data: Rc::new(RefCell::new(0)),
			cooldown,
		}
	}

	/// Return the number of times `poll()` has been called across all clones.
	#[must_use]
	pub fn get_poll_count(&self) -> usize {
		*self.data.borrow()
	}

	/// Create a shallow clone sharing the same internal data.
	///
	/// Call `get_poll_count()` on the clone to read counters updated
	/// when the original (or any other clone) is polled.
	#[must_use]
	pub fn shallow_clone(&self) -> Self {
		Self {
			data: self.data.clone(),
			cooldown: self.cooldown,
		}
	}
}

impl Plugin for TallyPlugin {
	fn poll(self: Box<Self>) -> (Duration, Box<dyn Plugin>) {
		*self.data.borrow_mut() += 1;
		(self.cooldown, self)
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_clone_shares_count() {
		let tally = TallyPlugin::default();
		let clone = tally.shallow_clone();

		assert_eq!(tally.get_poll_count(), 0);
		assert_eq!(clone.get_poll_count(), 0);

		let _ = Box::new(tally).poll();

		assert_eq!(clone.get_poll_count(), 1);
	}
}
