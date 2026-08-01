use std::fmt::Display;

// TODO: merge structs into enum
#[derive(Debug, Clone, PartialEq, Eq)]
struct StackFrame {
	message: String,
	file: &'static str,
	line: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct TrackLight {
	message: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum TracePoint {
	Heavy(StackFrame),
	Light(TrackLight),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ErrorSmart {
	stack: Vec<TracePoint>,
}

impl ErrorSmart {
	fn push_heavy_manual(&mut self, message: String, file: &'static str, line: u32) {
		let frame = StackFrame {
			message,
			file,
			line,
		};
		self.stack.push(TracePoint::Heavy(frame));
	}

	#[track_caller]
	pub fn new_heavy<T>(message: String) -> Result<T, Self> {
		let location = std::panic::Location::caller();
		Err(Self::new_heavy_manual(
			message,
			location.file(),
			location.line(),
		))
	}

	#[must_use]
	pub fn new_heavy_manual(message: String, file: &'static str, line: u32) -> Self {
		let mut obj = Self { stack: vec![] };
		obj.push_heavy_manual(message, file, line);
		obj
	}

	pub fn new_light<T>(message: &'static str) -> Result<T, Self> {
		let frame = TrackLight { message };
		Err(Self {
			stack: vec![TracePoint::Light(frame)],
		})
	}

	#[track_caller]
	pub fn push_heavy(&mut self, message: String) -> &mut Self {
		let location = std::panic::Location::caller();
		self.push_heavy_manual(message, location.file(), location.line());
		self
	}

	pub fn panic(&mut self) -> ! {
		// TODO: is this pretty?
		panic!("{}", self);
	}
}

/// `ResultSmart` is syntax sugar, but not in the way you might think.
///
/// It is NOT an alias for `Result<T, ErrorSmart>`; attempting to use it as such
/// will cause a build failure:
///
/// ```compile_fail
/// # use itj_tiny_deps::errors::ResultSmart;
/// fn main() -> ResultSmart<()> {
///     return Ok(());
/// }
/// ```
///
/// (TODO: I really should rename it to avoid this ambiguity... And add a
/// separate type alias that actually works that way)
///
/// Instead, it simply makes it simpler to add additional details to an error
/// before propagating it up the stack with `?`:
///
/// ```
/// # use itj_tiny_deps::errors::ResultSmart;
/// # use itj_tiny_deps::errors::ErrorSmart;
/// fn main() -> Result<(), ErrorSmart> {
///     let foo: Result<(), ErrorSmart> = Ok(());
///     let foo = foo.push_light("ABC failed while trying to XYZ")?;
///     return Ok(foo);
/// }
/// ```
pub trait ResultSmart {
	#[allow(dead_code)]
	fn push_light(self, message: &'static str) -> Self;

	#[track_caller]
	fn push_heavy(self, message: String) -> Self;
}

impl<T> ResultSmart for Result<T, ErrorSmart> {
	fn push_light(mut self, message: &'static str) -> Self {
		let Err(ref mut val) = self else {
			return self;
		};
		let frame = TrackLight { message };
		val.stack.push(TracePoint::Light(frame));
		self
	}

	#[track_caller]
	fn push_heavy(mut self, message: String) -> Self {
		let Err(ref mut val) = self else {
			return self;
		};
		let location = std::panic::Location::caller();
		val.push_heavy_manual(message, location.file(), location.line());
		self
	}
}

// TODO: Figure out why argh requires ErrorSmart to implement Display.
//
// ```
// error[E0277]: `itj_tiny_deps::errors::ErrorSmart` doesn't implement `std::fmt::Display`
//   --> itj_autolock/src/cli.rs:77:13
//    |
// 77 |     lock_type: LockType,
//    |                ^^^^^^^^ the trait `std::fmt::Display` is not implemented for `itj_tiny_deps::errors::ErrorSmart`
//    |
//    = note: required for `lockscreen::lockscreen_trait::LockType` to implement `argh::FromArgValue`
// ```
impl Display for ErrorSmart {
	fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
		write!(fmt, "ErrorSmart([").unwrap();
		for frame in &self.stack {
			match frame {
				TracePoint::Light(light) => write!(fmt, "Light({})", light.message).unwrap(),
				// TODO: add file & line?
				TracePoint::Heavy(heavy) => write!(fmt, "Heavy({})", heavy.message).unwrap(),
			}
		}
		write!(fmt, "])").unwrap();
		Ok(())
	}
}
