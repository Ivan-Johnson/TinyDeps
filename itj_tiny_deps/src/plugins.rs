use core::fmt::Debug;
use core::fmt::Formatter;
use std::thread;
use std::time::Duration;

pub trait Plugin {
	fn poll(&mut self);
}

/// A tiny synchronous plugin runner.
///
/// The host owns the plugin list and calls each plugin once per tick.
pub struct PluginRunner {
	plugins: Vec<Box<dyn Plugin>>,
}

impl Debug for PluginRunner {
	fn fmt(&self, fmt: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
		fmt.debug_struct("PluginRunner")
			.field("plugins", &self.plugins.len())
			.finish()
	}
}

impl PluginRunner {
	pub fn new(plugins: Vec<Box<dyn Plugin>>) -> Self {
		Self { plugins }
	}

	pub fn main(mut self) -> ! {
		loop {
			thread::sleep(Duration::from_millis(100));
			for plugin in &mut self.plugins {
				plugin.poll();
			}
		}
	}
}

// #[cfg(test)]
// mod tests {
// 	use super::*;
// 	use std::cell::RefCell;
// 	use std::rc::Rc;
//
// 	struct TestPlugin {
// 		counter: Rc<RefCell<u32>>,
// 	}
//
// 	impl Plugin for TestPlugin {
// 		fn poll(&mut self) {
// 			*self.counter.borrow_mut() += 1;
// 		}
// 	}
//
// 	#[test]
// 	fn stores_plugins() {
// 		let counter = Rc::new(RefCell::new(0));
// 		let runner = PluginRunner::new(vec![Box::new(TestPlugin {
// 			counter: counter.clone(),
// 		})]);
//
// 		assert_eq!(runner.plugins.len(), 1);
// 		assert_eq!(*counter.borrow(), 0);
// 	}
// }
//
