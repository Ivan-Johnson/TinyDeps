use core::fmt::Debug;
use core::time::Duration;
use std::time::Instant;

/// Minimal abstraction over time-related operations used throughout the crate.
///
/// This trait exists so production code can depend on a real clock while tests
/// can inject a deterministic mock implementation.
///
/// `now_instant` and `now_duration` intentionally serve different use cases:
/// `now_instant` is for monotonic interval measurement, while `now_duration` is
/// for callers that need a duration-style wall-clock timestamp.
pub trait Time: Debug {
	/// Sleep for the requested duration.
	///
	/// Real implementations normally block the current thread. Mock
	/// implementations typically advance simulated time instead.
	fn sleep(&self, dur: Duration);

	/// Return the current monotonic instant.
	///
	/// This is appropriate for measuring elapsed time and computing deadlines
	/// that should not be affected by wall-clock adjustments.
	fn now_instant(&self) -> Instant;

	/// Return the current wall-clock timestamp as a duration since a fixed epoch.
	///
	/// In the standard `RealTime` implementation this is the duration since the
	/// Unix epoch. Callers should prefer `now_instant` unless they specifically
	/// need a timestamp-like value.
	fn now_duration(&self) -> Duration;
}
