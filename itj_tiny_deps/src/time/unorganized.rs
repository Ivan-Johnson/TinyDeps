use core::time::Duration;
use std::process::Command;

pub const SECONDS_PER_MINUTE: u64 = 60;
pub const SECONDS_PER_HOUR: u64 = 60 * SECONDS_PER_MINUTE;
pub const SECONDS_PER_DAY: u64 = 24 * SECONDS_PER_HOUR;
// https://web.archive.org/web/20230506125056/https://www.grc.nasa.gov/www/k-12/Numbers/Math/Mathematical_Thinking/calendar_calculations.htm
pub const SECONDS_PER_TROPICAL_YEAR: u64 = SECONDS_PER_DAY * 365_2422 / 1_0000;

pub const SECONDS_PER_DAY_F64: f64 = SECONDS_PER_DAY as f64;

#[allow(dead_code)]
pub const DURATION_SECOND: Duration = Duration::from_secs(1);
#[allow(dead_code)]
pub const DURATION_MINUTE: Duration = Duration::from_secs(SECONDS_PER_MINUTE);
pub const DURATION_HOUR: Duration = Duration::from_secs(SECONDS_PER_HOUR);
pub const DURATION_DAY: Duration = Duration::from_secs(SECONDS_PER_DAY);
pub const DURATION_TROPICAL_YEAR: Duration = Duration::from_secs(SECONDS_PER_TROPICAL_YEAR);

/// The standard library's "duration" type doesn't support negative durations,
/// so we are left to implement basic addition functions ourselves -_-
pub fn duration_add_i64_secs(dur: Duration, secs: i64) -> Duration {
	if secs >= 0 {
		dur + Duration::from_secs(secs as u64)
	} else {
		dur - Duration::from_secs((-secs) as u64)
	}
}

// I make no guarentees about the format of this string.
//
// This function is purely for adding a timestamp to logs or some such.
pub fn get_date_as_string() -> String {
	let mut cmd = Command::new("date");
	let output = cmd.arg("--iso-8601=seconds").output().unwrap().stdout;
	let str = std::str::from_utf8(&output).unwrap();
	str[0..str.len() - 1].to_string()
}

pub fn round_duration_down(duration: Duration, multiple: u64) -> Duration {
	let secs = duration.as_secs();

	let remainder = secs % multiple;

	Duration::from_secs(secs - remainder)
}

pub fn round_duration_up(duration: Duration, multiple: u64) -> Duration {
	let secs = duration.as_secs();

	let remainder = secs % multiple;

	Duration::from_secs(secs + multiple - remainder)
}
