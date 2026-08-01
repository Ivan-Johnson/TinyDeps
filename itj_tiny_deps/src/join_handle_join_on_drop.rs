use std::thread::JoinHandle;

pub struct JoinHandleJoinOnDrop<T> {
	worker: Option<JoinHandle<T>>,
}

impl<T> JoinHandleJoinOnDrop<T> {
	#[must_use]
	pub const fn new(worker: JoinHandle<T>) -> Self {
		Self {
			worker: Some(worker),
		}
	}
}

impl<T> Drop for JoinHandleJoinOnDrop<T> {
	fn drop(&mut self) {
		self.worker.take().unwrap().join().unwrap();
	}
}
