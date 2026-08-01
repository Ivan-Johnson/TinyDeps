mod enqueue_plugin;
mod plugin_runner;
mod tally_plugin;

use core::time::Duration;

pub trait Plugin {
	// As a C developer, returning (Self, Duration) feels natural.
	//
	// However, in Rust (Duration, Self) seems to be more idiomatic:
	// (self.foo, self) works, but (self, self.foo) is an error.
	fn poll(self: Box<Self>) -> (Duration, Box<dyn Plugin>);
}

pub use enqueue_plugin::EnqueuePlugin;
pub use plugin_runner::PluginRunner;
pub use tally_plugin::TallyPlugin;
