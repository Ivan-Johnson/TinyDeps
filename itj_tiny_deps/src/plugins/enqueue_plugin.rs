use crate::plugins::Plugin;
use core::cell::RefCell;
use core::fmt::Debug;
use core::time::Duration;
use std::collections::VecDeque;
use std::rc::Rc;

/// A plugin that enqueues a value into a shared `VecDeque` on every `poll()`.
#[derive(Debug)]
pub struct EnqueuePlugin<T: Debug + Clone> {
	queue: Rc<RefCell<VecDeque<T>>>,
	value: T,
	cooldown: Duration,
}

impl<T: Debug + Clone> EnqueuePlugin<T> {
	/// Create a new enqueue plugin that pushes `value` into `queue` on each poll.
	pub fn new(queue: Rc<RefCell<VecDeque<T>>>, value: T, cooldown: Duration) -> Self {
		Self {
			queue,
			value,
			cooldown,
		}
	}
}

impl<T: Debug + Clone + 'static> Plugin for EnqueuePlugin<T> {
	fn poll(self: Box<Self>) -> (Box<dyn Plugin>, Duration) {
		self.queue.borrow_mut().push_back(self.value.clone());
		(
			Box::new(EnqueuePlugin {
				queue: self.queue.clone(),
				value: self.value,
				cooldown: self.cooldown,
			}),
			self.cooldown,
		)
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_basic_enqueue() {
		let queue: Rc<RefCell<VecDeque<char>>> = Rc::new(RefCell::new(VecDeque::new()));
		let plugin = EnqueuePlugin::new(queue.clone(), 'x', Duration::ZERO);

		let (_plugin, cooldown) = Box::new(plugin).poll();
		assert_eq!(cooldown, Duration::ZERO);
		assert_eq!(queue.borrow_mut().pop_front(), Some('x'));
	}

	#[test]
	fn test_shared_queue() {
		let queue: Rc<RefCell<VecDeque<&str>>> = Rc::new(RefCell::new(VecDeque::new()));
		let plugin_a = EnqueuePlugin::new(queue.clone(), "a", Duration::ZERO);
		let plugin_b = EnqueuePlugin::new(queue.clone(), "b", Duration::ZERO);

		let (_plugin_a, _) = Box::new(plugin_a).poll();
		let (_plugin_b, _) = Box::new(plugin_b).poll();

		assert_eq!(queue.borrow_mut().pop_front(), Some("a"));
		assert_eq!(queue.borrow_mut().pop_front(), Some("b"));
		assert!(queue.borrow().is_empty());
	}
}
