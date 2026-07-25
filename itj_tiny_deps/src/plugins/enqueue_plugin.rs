use crate::plugins::Plugin;
use core::cell::RefCell;
use core::fmt::Debug;
use std::collections::VecDeque;
use std::rc::Rc;

/// A plugin that enqueues a value into a shared `VecDeque` on every `poll()`.
#[derive(Debug)]
pub struct EnqueuePlugin<T: Debug + Clone> {
	queue: Rc<RefCell<VecDeque<T>>>,
	value: T,
}

impl<T: Debug + Clone> EnqueuePlugin<T> {
	/// Create a new enqueue plugin that pushes `value` into `queue` on each poll.
	pub fn new(queue: Rc<RefCell<VecDeque<T>>>, value: T) -> Self {
		Self { queue, value }
	}
}

impl<T: Debug + Clone + 'static> Plugin for EnqueuePlugin<T> {
	fn poll(self: Box<Self>) -> Box<dyn Plugin> {
		self.queue.borrow_mut().push_back(self.value.clone());
		Box::new(EnqueuePlugin {
			queue: self.queue.clone(),
			value: self.value,
		})
	}
}
