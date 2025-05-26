use crate::time::Time;
use core::fmt::Debug;
use core::time::Duration;
use std::time::Instant;
use std::time::SystemTime;
use std::time::UNIX_EPOCH;

#[derive(Default, Debug, Clone)]
pub struct RealTime {}

impl Time for RealTime {
	fn sleep(&self, dur: Duration) {
		std::thread::sleep(dur);
	}

	fn now_instant(&self) -> Instant {
		Instant::now()
	}

	fn now_duration(&self) -> Duration {
		SystemTime::now().duration_since(UNIX_EPOCH).unwrap()
	}
}
