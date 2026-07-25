use crate::plugins::Plugin;
use crate::time::Time;
use core::fmt::Debug;
use core::fmt::Formatter;
use std::thread;
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
