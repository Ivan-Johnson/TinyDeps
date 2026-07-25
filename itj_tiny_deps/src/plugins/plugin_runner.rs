use crate::plugins::Plugin;
use crate::time::Time;
use std::time::Duration;

// TODO: make a separate builder struct?
pub struct PluginRunner<T: Time> {
	pub plugins: Vec<Box<dyn Plugin>>,
	pub time: T,
	pub poll_frequency: Duration,
}

impl<T: Time> PluginRunner<T> {
	pub fn poll(mut self) -> Self {
		let old_plugins = std::mem::take(&mut self.plugins);
		for plugin in old_plugins {
			self.plugins.push(plugin.poll());
		}
		self
	}

	pub fn main(mut self) -> ! {
		loop {
			// TODO: do something smarter
			self.time.sleep(self.poll_frequency);

			self = self.poll();
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::plugins::TallyPlugin;
	use crate::time::MockTime;
	use core::time::Duration;

	const POLL_FREQUENCY: Duration = Duration::from_millis(100);

	#[test]
	fn test_poll_once() {
		let tally = TallyPlugin::new();
		let held = tally.shallow_clone();

		let runner = PluginRunner {
			plugins: vec![Box::new(tally)],
			time: MockTime::default(),
			poll_frequency: POLL_FREQUENCY,
		};

		let runner = runner.poll();

		assert_eq!(held.get_poll_count(), 1);
		assert_eq!(runner.plugins.len(), 1);
	}

	#[test]
	fn test_poll_twice() {
		let tally = TallyPlugin::new();
		let held = tally.shallow_clone();

		let runner = PluginRunner {
			plugins: vec![Box::new(tally)],
			time: MockTime::default(),
			poll_frequency: POLL_FREQUENCY,
		};

		let runner = runner.poll();
		let runner = runner.poll();

		assert_eq!(held.get_poll_count(), 2);
		assert_eq!(runner.plugins.len(), 1);
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
			poll_frequency: POLL_FREQUENCY,
		};

		let _runner = runner.poll();

		assert_eq!(held_a.get_poll_count(), 1);
		assert_eq!(held_b.get_poll_count(), 1);
	}
}
