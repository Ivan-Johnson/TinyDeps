use std::io::Read;
use std::io::Write;
use std::process::Child;
use std::process::ChildStderr;
use std::process::ChildStdin;
use std::process::ChildStdout;
use std::process::Command;
use std::process::Stdio;
use std::time::Duration;

use crate::ipc::base::IPC1;
use crate::TcpPort;

/// IIRC Linux guarantees that writes smaler than 4k are atomic; this size was
/// choosen accordingly.
const MAX_WRITE_SIZE: usize = 4_000;

pub struct IPCNC {
	builder: Command,
	child: Child,
	stdin: ChildStdin,
	stdout: ChildStdout,
	stderr: ChildStderr,
}

impl IPC1 for IPCNC {
	fn restart(&mut self) {
		self.wait_for_successful_exit(Duration::from_millis(50));

		let mut child = self.builder.spawn().expect("Failed to spawn nc");
		let (stdin, stdout, stderr) = Self::take_io(&mut child);

		self.child = child;
		self.stdin = stdin;
		self.stdout = stdout;
		self.stderr = stderr;
		self.assert_not_failed();
	}

	fn read(&mut self) -> Vec<u8> {
		self.assert_is_running();
		let mut buffer: [u8; MAX_WRITE_SIZE] = [0; MAX_WRITE_SIZE];
		let Ok(read_size) = self.stdout.read(&mut buffer) else {
			return vec![];
		};
		buffer[0..read_size].to_vec()
	}

	fn send(&mut self, msg: &Vec<u8>) {
		self.assert_is_running();
		println!("Writing {msg:?}");
		self.stdin.write_all(msg).unwrap();
	}
}

// TODO dedupe all the asserts
impl IPCNC {
	fn assert_is_running(&mut self) {
		let status = self
			.child
			.try_wait()
			.expect("Unable to determine if nc has exited or not??");
		assert_eq!(None, status, "ERROR: nc has exited");
	}

	fn wait_for_finish(&mut self, timeout: Duration) {
		let max_time = timeout;
		let cur_time = Duration::from_secs(0);
		while cur_time < max_time {
			let status = self
				.child
				.try_wait()
				.expect("Unable to determine if nc has exited or not??");
			if status.is_some() {
				// nc has exited; return.
				return;
			}

			std::thread::sleep(Duration::from_millis(10));
		}
		panic!("Timed out waiting for nc to finish")
	}

	fn wait_for_successful_exit(&mut self, timeout: Duration) {
		self.wait_for_finish(timeout);
		let status = self
			.child
			.try_wait()
			.expect("Unable to determine if nc has exited or not??")
			.expect("`wait_for_finish` returned successfully, so nc must have finished");
		if status.success() {
			return;
		}

		println!("ERROR: nc has crashed");

		// stdout
		let mut stdout_str = String::new();
		let stdout_ok = self.stdout.read_to_string(&mut stdout_str);
		if stdout_ok.is_ok() {
			println!("stdout: \"{stdout_str}\"");
		} else {
			println!("stdout not available");
		}

		// stderr
		let mut stderr_str = String::new();
		let stderr_ok = self.stderr.read_to_string(&mut stderr_str);
		if stderr_ok.is_ok() {
			println!("stderr: \"{stderr_str}\"");
		} else {
			println!("stderr not available");
		}

		// exit
		panic!();
	}

	fn assert_not_failed(&mut self) {
		let status = self
			.child
			.try_wait()
			.expect("Unable to determine if nc has exited or not??");
		let Some(status) = status else {
			// nc has not exited yet
			return;
		};
		if status.success() {
			// nc finished successfully
			return;
		};
		println!("ERROR: nc has crashed");

		// stdout
		let mut stdout_str = String::new();
		let stdout_ok = self.stdout.read_to_string(&mut stdout_str);
		if stdout_ok.is_ok() {
			println!("stdout: \"{stdout_str}\"");
		} else {
			println!("stdout not available");
		}

		// stderr
		let mut stderr_str = String::new();
		let stderr_ok = self.stderr.read_to_string(&mut stderr_str);
		if stderr_ok.is_ok() {
			println!("stderr: \"{stderr_str}\"");
		} else {
			println!("stderr not available");
		}

		// exit
		std::process::exit(1);
	}

	/// Return the child's stdin, stdout, and stderr pipes.
	fn take_io(child: &mut Child) -> (ChildStdin, ChildStdout, ChildStderr) {
		let stdin = child.stdin.take().expect("Failed to open stdin");
		let stdout = child.stdout.take().expect("Failed to open stdin");
		let stderr = child.stderr.take().expect("Failed to open stderr");
		(stdin, stdout, stderr)
	}

	fn spawn_with_io(mut builder: Command) -> Self {
		builder.stdin(Stdio::piped())
			.stdout(Stdio::piped())
			.stderr(Stdio::piped());
		println!("Running: {builder:?}");
		let mut child = builder.spawn().expect("Failed to spawn nc");
		let (stdin, stdout, stderr) = Self::take_io(&mut child);

		let mut obj = Self {
			builder,
			child,
			stdin,
			stdout,
			stderr,
		};
		obj.assert_not_failed();
		obj
	}

	pub fn open_server(port: TcpPort) -> Self {
		let mut builder = Command::new("nc");
		builder.arg("-l").arg(format!("{port}"));
		Self::spawn_with_io(builder)
	}

	pub fn open_client(port: TcpPort) -> Self {
		let mut builder = Command::new("nc");
		builder.arg("-N").arg("127.0.0.1").arg(format!("{port}"));
		Self::spawn_with_io(builder)
	}
}
