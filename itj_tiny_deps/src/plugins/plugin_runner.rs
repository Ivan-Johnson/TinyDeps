use crate::plugins::Plugin;
use crate::time::Time;
use core::time::Duration;
use std::time::Instant;

// TODO: get an AI agent to review all plugin-related tests.
//
// TODO: create plugin(s) to make `itj_tiny_deps::ipc::*` easier to use.
//
// TODO: update `itj_daemon_hello_world`

struct PluginEntry {
	plugin: Box<dyn Plugin>,
	deadline: Instant,
}

pub struct PluginRunner<T: Time> {
	entries: Vec<PluginEntry>,
	time: T,
	min_delay: Duration,
	max_delay: Duration,
}

impl<T: Time> PluginRunner<T> {
	pub fn new(time: T, plugins: Vec<Box<dyn Plugin>>, min_delay: Duration, max_delay: Duration) -> Self {
		let now = time.now_instant();
		let entries = plugins
			.into_iter()
			.map(|p| PluginEntry {
				plugin: p,
				deadline: now,
			})
			.collect();
		Self {
			entries,
			time,
			min_delay,
			max_delay,
		}
	}

	pub fn poll(mut self) -> (Self, Duration) {
		let mut next = self.time.now_instant() + self.max_delay;

		let tmp = Vec::with_capacity(self.entries.len());
		for entry in std::mem::replace(&mut self.entries, tmp) {
			if entry.deadline > self.time.now_instant() {
				// TODO: having two duplicate `self.entries.push` is very ugly.
				// Instead of manually pushing, I should use something like .into_inter().map().collect()
				self.entries.push(entry);
				continue;
			}

			let (cooldown, plugin) = entry.plugin.poll();
			let deadline = self.time.now_instant() + cooldown;
			next = std::cmp::min(next, deadline);
			self.entries.push(PluginEntry { plugin, deadline });
		}

		let delay = next.checked_duration_since(self.time.now_instant());
		let delay = delay.unwrap_or_default();
		let delay = delay.max(self.min_delay);
		(self, delay)
	}

	pub fn main(self) -> ! {
		// TODO: seriously? Why why do I have to use `me` instead of `self`?
		let mut me = self;

		loop {
			// TODO: there has to be a better way.
			let tmp = me.poll();
			me = tmp.0;
			let delay = tmp.1;

			me.time.sleep(delay);
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::plugins::EnqueuePlugin;
	use crate::plugins::TallyPlugin;
	use crate::time::MockTime;
	use core::time::Duration;
	use std::cell::RefCell;
	use std::collections::VecDeque;
	use std::rc::Rc;

	const MIN_DELAY: Duration = Duration::from_millis(1);
	const MAX_DELAY: Duration = Duration::from_secs(100);

	#[test]
	fn test_poll_once() {
		let time = MockTime::default();
		let tally = TallyPlugin::new(Duration::ZERO);
		let held = tally.shallow_clone();

		let runner = PluginRunner::new(
			time.shallow_clone(),
			vec![Box::new(tally)],
			MIN_DELAY,
			MAX_DELAY,
		);
		let (runner, _delay) = runner.poll();

		assert_eq!(held.get_poll_count(), 1);
		assert_eq!(runner.entries.len(), 1);
	}

	#[test]
	fn test_poll_twice() {
		let time = MockTime::default();
		let tally = TallyPlugin::new(Duration::ZERO);
		let held = tally.shallow_clone();

		let runner = PluginRunner::new(
			time.shallow_clone(),
			vec![Box::new(tally)],
			MIN_DELAY,
			MAX_DELAY,
		);
		let (runner, _delay) = runner.poll();

		// Advance time so deadline passes
		runner.time.sleep(Duration::from_millis(1));
		let (runner, _delay) = runner.poll();

		assert_eq!(held.get_poll_count(), 2);
		assert_eq!(runner.entries.len(), 1);
	}

	#[test]
	fn test_poll_order() {
		let time = MockTime::default();
		let queue: Rc<RefCell<VecDeque<char>>> = Rc::new(RefCell::new(VecDeque::new()));
		let enqueue_a = EnqueuePlugin::new(queue.clone(), 'a', Duration::ZERO);
		let enqueue_b = EnqueuePlugin::new(queue.clone(), 'b', Duration::ZERO);

		let runner = PluginRunner::new(
			time.shallow_clone(),
			vec![Box::new(enqueue_a), Box::new(enqueue_b)],
			MIN_DELAY,
			MAX_DELAY,
		);
		let (runner, _delay) = runner.poll();

		assert_eq!(queue.borrow_mut().pop_front(), Some('a'));
		assert_eq!(queue.borrow_mut().pop_front(), Some('b'));
		assert!(queue.borrow().is_empty());

		runner.time.sleep(Duration::from_millis(1));
		let _runner = runner.poll();

		assert_eq!(queue.borrow_mut().pop_front(), Some('a'));
		assert_eq!(queue.borrow_mut().pop_front(), Some('b'));
		assert!(queue.borrow().is_empty());
	}

	#[test]
	fn test_plugin_not_polled_before_deadline() {
		let time = MockTime::default();
		let tally = TallyPlugin::new(Duration::from_secs(10));
		let held = tally.shallow_clone();

		let runner = PluginRunner::new(
			time.shallow_clone(),
			vec![Box::new(tally)],
			MIN_DELAY,
			MAX_DELAY,
		);

		// First poll: deadline is now, so it gets polled
		let (runner, _delay) = runner.poll();
		assert_eq!(held.get_poll_count(), 1);

		// Second poll at same instant: deadline is now + 10s, should NOT poll
		let _runner = runner.poll();
		assert_eq!(held.get_poll_count(), 1);
	}

	#[test]
	fn test_plugin_polled_after_deadline() {
		let time = MockTime::default();
		let tally = TallyPlugin::new(Duration::from_secs(5));
		let held = tally.shallow_clone();

		let runner = PluginRunner::new(
			time.shallow_clone(),
			vec![Box::new(tally)],
			MIN_DELAY,
			MAX_DELAY,
		);

		// First poll
		let (runner, _delay) = runner.poll();
		assert_eq!(held.get_poll_count(), 1);

		// Advance time past deadline
		time.sleep(Duration::from_secs(5));
		let _runner = runner.poll();
		assert_eq!(held.get_poll_count(), 2);
	}

	#[test]
	fn test_zero_duration_repoll() {
		let time = MockTime::default();
		let tally = TallyPlugin::new(Duration::ZERO);
		let held = tally.shallow_clone();

		let mut runner = PluginRunner::new(
			time.shallow_clone(),
			vec![Box::new(tally)],
			MIN_DELAY,
			MAX_DELAY,
		);

		// Poll three times, each after advancing time slightly
		for _ in 0..3 {
			runner.time.sleep(Duration::from_millis(1));
			(runner, _) = runner.poll();
		}

		assert_eq!(held.get_poll_count(), 3);
	}
}
