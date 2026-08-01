use crate::errors::ErrorSmart;
use crate::http::http_core::HttpGetResponse;
use crate::http::Http;
use crate::time::Time;
use std::cell::Cell;
use std::fmt::Debug;
use std::rc::Rc;
use std::time::Duration;
use std::time::Instant;

/// A wrapper around the HTTP interface to throttle the number of requests that
/// are made.
#[derive(Debug)]
pub struct LeakyBucket<T: Time> {
	base_impl: Rc<dyn Http>,
	bucket_size: u32,
	query_delay: Duration,
	time: T,

	/// The time at which the next request can be made
	threshold: Cell<Instant>,
}

impl<T: Time> LeakyBucket<T> {
	fn get_default_threshold(&self) -> Instant {
		self.time
			.now_instant()
			.checked_sub(self.query_delay * (self.bucket_size - 1))
			.unwrap()
	}

	pub fn new(base_impl: Rc<dyn Http>, time: T, bucket_size: u32, query_delay: Duration) -> Self {
		let threshold = Cell::new(time.now_instant());

		let obj = Self {
			base_impl,
			bucket_size,
			query_delay,
			time,
			threshold,
		};

		obj.threshold.set(obj.get_default_threshold());
		obj
	}
}

impl<T: Time> Http for LeakyBucket<T> {
	fn get(&self, url: &str) -> Result<HttpGetResponse, ErrorSmart> {
		let threshold_init = std::cmp::max(self.threshold.get(), self.get_default_threshold());
		let now = self.time.now_instant();
		if now < threshold_init {
			self.time.sleep(threshold_init - now);
		}
		self.threshold.set(threshold_init + self.query_delay);
		self.base_impl.get(url)
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::http::MockHttpController;
	use std::fmt::Debug;
	use std::iter::Iterator;
	use std::rc::Rc;
	use std::time::Duration;

	struct TestOperationConstructorArgs {
		bucket_size: u32,
		query_delay: Duration,
	}

	#[derive(Debug)]
	struct TestOperationGet {
		/// Time at which instant is called, relative to the start of the test
		t_enqueue: Duration,
		/// Time at the leaky bucket is expected to call the underlying HTTP get
		/// function, relative to the start of the test
		t_dequeue: Duration,
		/// True if the operation succeeds, false if it returns an error
		success: bool,
	}

	fn run_test(args: TestOperationConstructorArgs, operations: impl Iterator<Item = TestOperationGet>) {
		let mut http_controller = MockHttpController::default();
		let time_mock = http_controller.get_mock_time().shallow_clone();
		let time_start: Instant = time_mock.now_instant();

		let operations: Vec<_> = operations.collect();

		// For the purposes of this test, we only care about:
		// * the time the `get` function is called at
		// * whether or not the `get` function succeeds
		//
		// Since we don't care about these other values, they always have the same values:
		let url = "asdf".to_string();
		let expected_err = ErrorSmart::new_light("asdf");
		let expected_ok: Result<HttpGetResponse, ErrorSmart> = Ok(HttpGetResponse {
			headers: vec![],
			body: "asdf".to_string(),
		});
		for TestOperationGet {
			t_enqueue: _,
			t_dequeue,
			success,
		} in &operations
		{
			let expected_response = if *success {
				expected_ok.clone()
			} else {
				expected_err.clone()
			};
			http_controller = http_controller.add_timed_expected_fn_call(
				time_start + *t_dequeue,
				url.clone(),
				expected_response,
			);
		}
		let http_mock = Rc::new(http_controller.to_impl());

		let bucket = LeakyBucket::new(
			http_mock,
			time_mock.shallow_clone(),
			args.bucket_size,
			args.query_delay,
		);

		for TestOperationGet {
			t_enqueue,
			t_dequeue,
			success,
		} in &operations
		{
			let expected_response = if *success {
				expected_ok.clone()
			} else {
				expected_err.clone()
			};
			let t_enqueue = time_start + *t_enqueue;
			let t_dequeue = time_start + *t_dequeue;
			assert!(t_enqueue <= t_dequeue);

			println!("Running {t_enqueue:?}, {t_dequeue:?}, {success}");

			let t_now = time_mock.now_instant();
			assert!(t_now <= t_enqueue);
			let t_sleep = t_enqueue - t_now;
			time_mock.sleep(t_sleep);
			assert!(time_mock.now_instant() == t_enqueue);

			let actual_response = bucket.get(&url);
			assert_eq!(expected_response, actual_response);
		}
	}

	// Test the trivial case where no sleeping occurs
	#[test]
	fn test_trivial() {
		let args = TestOperationConstructorArgs {
			bucket_size: 3,
			query_delay: Duration::from_secs(10),
		};
		let operations = [
			TestOperationGet {
				t_enqueue: Duration::from_secs(0),
				t_dequeue: Duration::from_secs(0),
				success: true,
			},
			TestOperationGet {
				t_enqueue: Duration::from_secs(0),
				t_dequeue: Duration::from_secs(0),
				success: true,
			},
			TestOperationGet {
				t_enqueue: Duration::from_secs(0),
				t_dequeue: Duration::from_secs(0),
				success: true,
			},
		];
		run_test(args, operations.into_iter());
	}

	// Test the basic case where there is an instantanous burst in traffic that is
	// larger than the bucket size
	#[test]
	fn test_burst_instantanous() {
		let args = TestOperationConstructorArgs {
			bucket_size: 3,
			query_delay: Duration::from_secs(10),
		};
		let operations = [
			TestOperationGet {
				t_enqueue: Duration::from_secs(0),
				t_dequeue: Duration::from_secs(0),
				success: true,
			},
			TestOperationGet {
				t_enqueue: Duration::from_secs(0),
				t_dequeue: Duration::from_secs(0),
				success: true,
			},
			TestOperationGet {
				t_enqueue: Duration::from_secs(0),
				t_dequeue: Duration::from_secs(0),
				success: true,
			},
			TestOperationGet {
				t_enqueue: Duration::from_secs(0),
				t_dequeue: Duration::from_secs(10),
				success: true,
			},
			TestOperationGet {
				t_enqueue: Duration::from_secs(10),
				t_dequeue: Duration::from_secs(20),
				success: true,
			},
		];
		run_test(args, operations.into_iter());
	}

	// Test the basic case where there is a near instantanous burst in traffic that
	// is larger than the bucket size
	#[test]
	fn test_near_burst() {
		let args = TestOperationConstructorArgs {
			bucket_size: 3,
			query_delay: Duration::from_secs(10),
		};
		let operations = [
			TestOperationGet {
				t_enqueue: Duration::from_secs(0),
				t_dequeue: Duration::from_secs(0),
				success: true,
			},
			TestOperationGet {
				t_enqueue: Duration::from_secs(1),
				t_dequeue: Duration::from_secs(1),
				success: true,
			},
			TestOperationGet {
				t_enqueue: Duration::from_secs(2),
				t_dequeue: Duration::from_secs(2),
				success: true,
			},
			TestOperationGet {
				t_enqueue: Duration::from_secs(3),
				t_dequeue: Duration::from_secs(10),
				success: true,
			},
			TestOperationGet {
				t_enqueue: Duration::from_secs(11),
				t_dequeue: Duration::from_secs(20),
				success: true,
			},
		];
		run_test(args, operations.into_iter());
	}

	/// Verify that only `bucket_size` charges are accumulated after a long delay
	#[test]
	fn test_overcharge() {
		let args = TestOperationConstructorArgs {
			bucket_size: 3,
			query_delay: Duration::from_secs(10),
		};
		let operations = [
			TestOperationGet {
				t_enqueue: Duration::from_secs(0),
				t_dequeue: Duration::from_secs(0),
				success: true,
			},
			TestOperationGet {
				t_enqueue: Duration::from_secs(1),
				t_dequeue: Duration::from_secs(1),
				success: true,
			},
			TestOperationGet {
				t_enqueue: Duration::from_secs(2),
				t_dequeue: Duration::from_secs(2),
				success: true,
			},
			TestOperationGet {
				t_enqueue: Duration::from_secs(3),
				t_dequeue: Duration::from_secs(10),
				success: true,
			},
			// bucket is now fully depleated

			// Wait a long time, then verify that only three requests can be made without delay
			TestOperationGet {
				t_enqueue: Duration::from_secs(100),
				t_dequeue: Duration::from_secs(100),
				success: true,
			},
			TestOperationGet {
				t_enqueue: Duration::from_secs(101),
				t_dequeue: Duration::from_secs(101),
				success: true,
			},
			TestOperationGet {
				t_enqueue: Duration::from_secs(102),
				t_dequeue: Duration::from_secs(102),
				success: true,
			},
			TestOperationGet {
				t_enqueue: Duration::from_secs(103),
				t_dequeue: Duration::from_secs(110),
				success: true,
			},
		];
		run_test(args, operations.into_iter());
	}

	/// Run a test where at least one "charge" has been accumulated in the time it
	/// takes to deplete all of the charges
	#[test]
	fn test_slow_depletion() {
		let args = TestOperationConstructorArgs {
			bucket_size: 3,
			query_delay: Duration::from_secs(10),
		};
		let operations = [
			// Below counts are the number of operations that could be
			// completed instantaneously at the time the previous operation
			// completed

			// count: 3
			TestOperationGet {
				t_enqueue: Duration::from_secs(0),
				t_dequeue: Duration::from_secs(0),
				success: true,
			},
			// count: 2
			TestOperationGet {
				t_enqueue: Duration::from_secs(1),
				t_dequeue: Duration::from_secs(1),
				success: true,
			},
			// count: 1
			TestOperationGet {
				t_enqueue: Duration::from_secs(12),
				t_dequeue: Duration::from_secs(12),
				success: true,
			},
			// count: 1
			TestOperationGet {
				t_enqueue: Duration::from_secs(20),
				t_dequeue: Duration::from_secs(20),
				success: true,
			},
			// count: 1
			TestOperationGet {
				t_enqueue: Duration::from_secs(21),
				t_dequeue: Duration::from_secs(21),
				success: true,
			},
			// count: 0
			TestOperationGet {
				t_enqueue: Duration::from_secs(22),
				t_dequeue: Duration::from_secs(30),
				success: true,
			},
			// count: 0
		];
		run_test(args, operations.into_iter());
	}
}
