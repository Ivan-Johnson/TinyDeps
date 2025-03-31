use crate::random::Random;
use core::time::Duration;
use std::fs::File;
use std::io::Read;
use std::io::Result;

/// An implementation of Random that works by reading random bytes from a file.
pub struct RandomFromFile {
	file: File,
}

impl Default for RandomFromFile {
	/// This function always uses /dev/urandom, which is a non-blocking
	/// pseudorandom number generator.
	///
	/// In the future we could add support for /dev/random which
	/// on *some* machines provides truely random numbers.
	fn default() -> Self {
		let file = File::open("/dev/urandom").unwrap();
		Self { file }
	}
}

impl RandomFromFile {
	fn rand_u64(&mut self) -> Result<u64> {
		let mut buffer = [0u8; 8]; // 64 bits
		self.file.read_exact(&mut buffer)?;
		Ok(u64::from_le_bytes(buffer))
	}
}

impl Random for RandomFromFile {
	/// Generate a random number between min (inclusive) and max
	/// (exclusive).
	fn rand_int(&mut self, min: i64, max: i64) -> i64 {
		assert!(max > min);
		let range: u64 = (max - min).try_into().unwrap();

		loop {
			let rand_u64: u64 = self.rand_u64().unwrap();

			// If the random value is largest than the biggest
			// multiple of `range` then we need to throw it away and
			// try again, otherwise our distribution won't be
			// uniform.
			let biggest_multiple = (u64::MAX / range) * range;
			if rand_u64 >= biggest_multiple {
				continue;
			}
			let rand_range: i64 = (rand_u64 % range).try_into().unwrap();
			return min + rand_range;
		}
	}

	fn rand_duration(&mut self, max: Duration) -> Duration {
		let rand_max_i = max.as_secs();
		let rand = self.rand_int(0, rand_max_i.try_into().unwrap());
		Duration::from_secs(rand.try_into().unwrap())
	}

	fn rand_f64(&mut self, max: f64) -> f64 {
		const SCALE: i64 = i64::MAX;
		let result_i64 = self.rand_int(0, SCALE);
		let frac = (result_i64 as f64) / (SCALE as f64);
		frac * max
	}
}
