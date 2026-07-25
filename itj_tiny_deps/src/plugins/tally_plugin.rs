use crate::plugins::Plugin;
use core::cell::RefCell;
use core::fmt::Debug;
use std::rc::Rc;

impl Default for TallyPlugin {
	fn default() -> Self {
		Self::new()
	}
}

/// A minimal `Plugin` implementation that tracks how many times `poll()` has been called.
#[derive(Debug)]
pub struct TallyPlugin {
	data: Rc<RefCell<usize>>,
}

impl TallyPlugin {
	/// Create a fresh tally plugin with `poll_count` = 0.
	pub fn new() -> Self {
		Self {
			data: Rc::new(RefCell::new(0)),
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
		}
	}
}

impl Plugin for TallyPlugin {
	fn poll(self: Box<Self>) -> Box<dyn Plugin> {
		*self.data.borrow_mut() += 1;
		Box::new(TallyPlugin {
			data: self.data.clone(),
		})
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::plugins::PluginRunner;
	use crate::time::MockTime;
	use core::time::Duration;

	#[test]
	fn test_poll_count_increments_once() {
		let tally = TallyPlugin::new();
		let held = tally.shallow_clone();
		assert_eq!(tally.get_poll_count(), 0);

		let _ = Box::new(tally).poll();
		assert_eq!(held.get_poll_count(), 1);
	}

	#[test]
	fn test_clone_shares_count() {
		let tally = TallyPlugin::new();
		let clone = tally.shallow_clone();

		assert_eq!(tally.get_poll_count(), 0);
		assert_eq!(clone.get_poll_count(), 0);

		let _ = Box::new(tally).poll();

		assert_eq!(clone.get_poll_count(), 1);
	}

	#[test]
	fn test_plugin_runner_polls_once() {
		let tally = TallyPlugin::new();
		let held = tally.shallow_clone();

		let runner = PluginRunner {
			plugins: vec![Box::new(tally)],
			time: MockTime::default(),
			poll_frequency: Duration::from_millis(100),
		};

		let runner = runner.poll();

		assert_eq!(held.get_poll_count(), 1);
		assert_eq!(runner.plugins.len(), 1);
	}

	#[test]
	fn test_plugin_runner_polls_twice() {
		let tally = TallyPlugin::new();
		let held = tally.shallow_clone();

		let runner = PluginRunner {
			plugins: vec![Box::new(tally)],
			time: MockTime::default(),
			poll_frequency: Duration::ZERO,
		};

		let runner = runner.poll();
		let _runner = runner.poll();

		assert_eq!(held.get_poll_count(), 2);
	}

	#[test]
	fn test_multiple_plugins_independent() {
		let tally_a = TallyPlugin::new();
		let tally_b = TallyPlugin::new();
		let held_a = tally_a.shallow_clone();
		let held_b = tally_b.shallow_clone();

		let runner = PluginRunner {
			plugins: vec![Box::new(tally_a), Box::new(tally_b)],
			time: MockTime::default(),
			poll_frequency: Duration::ZERO,
		};

		let _runner = runner.poll();

		assert_eq!(held_a.get_poll_count(), 1);
		assert_eq!(held_b.get_poll_count(), 1);
	}
}
