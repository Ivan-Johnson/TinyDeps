#[cfg(feature = "random")]
mod test_random {

	use core::fmt::Debug;
	use itj_tiny_deps::random::Random;
	use itj_tiny_deps::random::RandomFromFile;

	enum ValueAssesment {
		Invalid,
		Low,
		Okay,
		High,
	}

	fn run_test<T: Debug + std::marker::Copy>(
		get_rand_val: &mut impl FnMut() -> T,
		assess: &impl Fn(T) -> ValueAssesment,
		num_trials: i32,
	) {
		let mut num_low = 0;
		let mut num_high = 0;

		for _ in 0..num_trials {
			let value = get_rand_val();
			match assess(value) {
				ValueAssesment::Invalid => {
					panic!("fuck. {value:?}");
				}
				ValueAssesment::Low => num_low += 1,
				ValueAssesment::High => num_high += 1,
				ValueAssesment::Okay => (),
			}
		}

		assert!(num_low > 0);
		assert!(num_high > 0);
	}

	#[test]
	fn test_trivial() {
		let mut rand = RandomFromFile::default();

		let width = 5.0;
		let threshold = 0.1;

		let mut get_f64 = || rand.rand_f64(width);
		let assess = |val| {
			if val < 0.0 {
				ValueAssesment::Invalid
			} else if val < threshold {
				ValueAssesment::Low
			} else if val <= width - threshold {
				ValueAssesment::Okay
			} else if val < width {
				ValueAssesment::High
			} else {
				ValueAssesment::Invalid
			}
		};

		run_test(&mut get_f64, &assess, 1_000_000);
	}
}
