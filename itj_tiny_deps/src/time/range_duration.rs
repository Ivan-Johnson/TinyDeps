use crate::random::Random;
use crate::random::RandomFromFile;
use core::time::Duration;

pub type GetDurationCallback<'a> = &'a dyn Fn() -> Duration;

#[derive(Clone)]
pub enum RangeDurationStop<'a> {
	Fixed(Duration),
	Callback(GetDurationCallback<'a>),
}

impl RangeDurationStop<'_> {
	fn get_value(&mut self) -> Duration {
		match self {
			RangeDurationStop::Fixed(value) => *value,
			RangeDurationStop::Callback(cb) => cb(),
		}
	}
}

/// Similar to `std::ops::Range`, but for Durations.
///
/// Also adds support for randomization.
struct RangeDuration<'a> {
	next: Duration,
	end: RangeDurationStop<'a>,
	step: Duration,
	step_rand_offset: Duration,
	step_rand_drift: Duration,
	rand: RandomFromFile,
}

impl Iterator for RangeDuration<'_> {
	type Item = Duration;
	fn next(&mut self) -> Option<<Self as Iterator>::Item> {
		let ret = self.next + self.rand.rand_duration(self.step_rand_offset);
		self.next += self.step + self.rand.rand_duration(self.step_rand_drift);
		if ret < self.end.get_value() {
			Some(ret)
		} else {
			None
		}
	}
}

#[derive(Clone)]
pub struct RangeDurationArgs<'a> {
	pub start: Duration,
	pub step: Duration,
	pub stop: RangeDurationStop<'a>,
}

#[derive(Clone)]
pub struct RandomizedRangeDurationArgs<'a> {
	pub args_nonrandom: RangeDurationArgs<'a>,
	pub step_rand_offset: Duration,
	pub step_rand_drift: Duration,
	pub do_random_pre_step: bool,
}

// I'm deliberately having this be a standalone function; there's no reason to
// expose implementation details if I don't have to.
pub fn new_range_duration(args: RangeDurationArgs<'_>) -> impl Iterator<Item = Duration> + '_ {
	// RAND%1 => no random step.
	let rand_dur = Duration::from_secs(1);
	let args = RandomizedRangeDurationArgs {
		args_nonrandom: args,
		step_rand_offset: rand_dur,
		step_rand_drift: rand_dur,
		do_random_pre_step: false,
	};
	new_randomized_range_duration(args)
}

pub fn new_randomized_range_duration(args: RandomizedRangeDurationArgs<'_>) -> impl Iterator<Item = Duration> + '_ {
	let mut rand = RandomFromFile::default();
	let mut next = args.args_nonrandom.start;
	if args.do_random_pre_step {
		// I don't think we need to do all three here, but it doesn't hurt either?
		next += rand.rand_duration(args.args_nonrandom.step);
		next += rand.rand_duration(args.step_rand_offset);
		next += rand.rand_duration(args.step_rand_drift);
	}
	RangeDuration {
		next,
		end: args.args_nonrandom.stop,
		step: args.args_nonrandom.step,
		step_rand_offset: args.step_rand_offset,
		step_rand_drift: args.step_rand_drift,
		rand,
	}
}
