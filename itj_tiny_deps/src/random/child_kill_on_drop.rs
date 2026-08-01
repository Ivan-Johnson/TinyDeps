use std::process::Child;

pub struct ChildKillOnDrop {
	child: Child,
}

impl ChildKillOnDrop {
	pub const fn new(child: Child) -> Self {
		Self { child }
	}
}

impl Drop for ChildKillOnDrop {
	fn drop(&mut self) {
		self.child.kill().unwrap();
		self.child.wait().unwrap();
	}
}
