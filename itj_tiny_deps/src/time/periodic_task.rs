use core::fmt::Debug;
use core::fmt::Formatter;
use core::time::Duration;
use std::time::Instant;

/// A trivial module that calls a callback function at fixed intervals.
///
/// This is NOT multithreaded or anything fancy like that; you have to
/// periodically call the run function, which will then invoke the callback if
/// sufficient time has passed.
pub struct PeriodicTask<'a, T> {
	callback: &'a dyn Fn(T),
	t_next: Instant,
	period: Duration,
}

impl<'a, T> Debug for PeriodicTask<'a, T> {
	fn fmt(&self, fmt: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
		fmt.debug_struct("PeriodicTask")
			.field("callback", &"???")
			.field("t_next", &self.t_next)
			.field("period", &self.period)
			.finish()
	}
}

impl<'a, T> PeriodicTask<'a, T> {
	// TODO: add a `Time` argument here?
	pub fn new(callback: &'a dyn Fn(T), period: Duration) -> Self {
		PeriodicTask {
			callback,
			t_next: Instant::now(),
			period,
		}
	}

	pub fn run(&mut self, args: T) {
		let now = Instant::now();
		if self.t_next < now {
			(self.callback)(args);
			self.t_next = now + self.period;
		}
	}
}
