use core::fmt::Debug;
use core::time::Duration;
use std::time::Instant;

pub trait Time: Debug {
	fn sleep(&self, dur: Duration);
	fn now_instant(&self) -> Instant;
	fn now_duration(&self) -> Duration;
}
