mod enqueue_plugin;
mod plugin_runner;
mod tally_plugin;

pub trait Plugin {
	fn poll(self: Box<Self>) -> Box<dyn Plugin>;
}

pub use enqueue_plugin::EnqueuePlugin;
pub use plugin_runner::PluginRunner;
pub use tally_plugin::TallyPlugin;
