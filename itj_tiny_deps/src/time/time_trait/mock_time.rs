use crate::time::Time;
use core::cell::RefCell;
use core::fmt::Debug;
use core::time::Duration;
use std::rc::Rc;
use std::time::Instant;

#[derive(Debug)]
struct MockTimeData {
	start_i: Instant,
	delta: RefCell<Duration>,
}

#[derive(Debug)]
pub struct MockTime {
	data: Rc<MockTimeData>,
}

impl Default for MockTime {
	fn default() -> Self {
		let data = Rc::new(MockTimeData {
			start_i: Instant::now(),
			delta: RefCell::new(Duration::from_secs(100_000)),
		});
		Self { data }
	}
}

impl Time for MockTime {
	fn sleep(&self, dur: Duration) {
		let data = &self.data;
		*data.delta.borrow_mut() += dur;
	}

	fn now_instant(&self) -> Instant {
		let data = &self.data;
		data.start_i + *data.delta.borrow()
	}

	fn now_duration(&self) -> Duration {
		*self.data.delta.borrow()
	}
}

impl MockTime {
	/// Create a shallow clone of Self.
	///
	/// When a sleep occurs on one of the clones, time will also advance for
	/// the other clone.
	///
	/// The clone *cannot* be sent to a different thread. However, this is
	/// still useful so that an integration test can advance time in between
	/// successive function calls.
	#[allow(unused)]
	pub fn shallow_clone(&self) -> Self {
		Self {
			data: self.data.clone(),
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	const MAX_ERROR_MOCK: Duration = Duration::from_nanos(0);
	/// 2025-01-29: I've consistently been seeing errors of ~1.1us, and am
	/// therefore increasing max error from 1us to 10us
	const MAX_ERROR_REAL: Duration = Duration::from_nanos(10_000);
	const SLEEP_DURATION: Duration = Duration::from_secs(100);

	#[test]
	fn test_basic_sleep() {
		// Setup
		let time = MockTime::default();

		// Save start times
		let start_dur = time.now_duration();
		let start_instant = time.now_instant();
		let realtime_start = Instant::now();

		// Sleep
		time.sleep(SLEEP_DURATION);

		// Save stop times
		let realtime_stop = Instant::now();
		let stop_dur = time.now_duration();
		let stop_instant = time.now_instant();

		// Validate results
		let delta_d = stop_dur - start_dur;
		println!("delta_d={delta_d:?}");
		assert!(SLEEP_DURATION.abs_diff(delta_d) <= MAX_ERROR_MOCK);
		let delta_i = stop_instant - start_instant;
		println!("delta_i={delta_i:?}");
		assert!(SLEEP_DURATION.abs_diff(delta_i) <= MAX_ERROR_MOCK);
		let delta_real = realtime_stop - realtime_start;
		println!("delta_real={delta_real:?}");
		assert!(delta_real <= MAX_ERROR_REAL);
	}

	/// Verify that calling sleep on a clone will cause time to advance on
	/// the original, and visa-versa.
	#[test]
	fn test_clone() {
		// Setup
		let orig = MockTime::default();
		let start = orig.now_duration();
		let clone = orig.shallow_clone();

		// Run test
		orig.sleep(SLEEP_DURATION);
		clone.sleep(SLEEP_DURATION * 2);
		let expected_stop = start + 3 * SLEEP_DURATION;
		let stop_orig = orig.now_duration();
		let stop_clone = clone.now_duration();

		// Validate original
		let delta_orig = expected_stop.abs_diff(stop_orig);
		println!("delta_orig={delta_orig:?}");
		assert!(delta_orig <= MAX_ERROR_MOCK);

		// Validate clone
		let delta_clone = expected_stop.abs_diff(stop_clone);
		println!("delta_clone={delta_clone:?}");
		assert!(delta_clone <= MAX_ERROR_MOCK);
	}
}
