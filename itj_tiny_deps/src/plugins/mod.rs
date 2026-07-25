mod mock_plugin;
mod plugin_runner;

pub trait Plugin {
	fn poll(self: Box<Self>) -> Box<dyn Plugin>;
}

pub use mock_plugin::MockPlugin;
pub use plugin_runner::PluginRunner;
