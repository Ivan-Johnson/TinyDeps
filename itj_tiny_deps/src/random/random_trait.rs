use core::time::Duration;

// TODO deduplicate by having separate RandomBytes trait, that generates N random
// bytes. Then multiple RNG sources can reuse the same logic for converting random
// bytes to random ints, floats, duration, etc.

/// Trait that provides a method to generate a random integer.
pub trait Random {
	/// Generates a random integer following from
	/// a uniform distribution on [min, max).
	fn rand_int(&mut self, min: i64, max: i64) -> i64;
	fn rand_duration(&mut self, max: Duration) -> Duration;
	fn rand_f64(&mut self, max: f64) -> f64;
}
