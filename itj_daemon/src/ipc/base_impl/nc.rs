use std::io::Read;
use std::process::Child;
use std::process::ChildStderr;
use std::process::ChildStdin;
use std::process::ChildStdout;
use std::process::Command;
use std::process::Stdio;

use crate::ipc::base::IPC1;
use crate::TcpPort;

/// IIRC Linux guarantees that writes smaler than 4k are atomic; this size was
/// choosen accordingly.
const MAX_WRITE_SIZE: usize = 4_000;

pub struct IPCNC {
	child: Child,
	_stdin: ChildStdin,
	stdout: ChildStdout,
	stderr: ChildStderr,
}

impl IPC1 for IPCNC {
	fn read(&mut self) -> Vec<u8> {
		self.assert_is_running();
		let mut buffer: [u8; MAX_WRITE_SIZE] = [0; MAX_WRITE_SIZE];
		let Ok(read_size) = self.stdout.read(&mut buffer) else {
			return vec![];
		};
		buffer[0..read_size].to_vec()
	}

	fn send(&mut self, _msg: &Vec<u8>) {
		self.assert_not_failed();
		todo!()
	}
}

impl IPCNC {
	fn assert_is_running(&mut self) {
		let status = self
			.child
			.try_wait()
			.expect("Unable to determine if nc has exited or not??");
		assert_eq!(None, status, "ERROR: nc has exited");
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

	pub fn open_server(port: TcpPort) -> Self {
		let mut builder = Command::new("nc");
		builder.arg("-l")
			.arg(format!("{port}"))
			.stdin(Stdio::piped())
			.stdout(Stdio::piped())
			.stderr(Stdio::piped());
		println!("nc builder: {builder:?}");
		let mut child = builder.spawn().expect("Failed to launch `nc` server");
		let _stdin = child.stdin.take().expect("Failed to open stdin");
		let stdout = child.stdout.take().expect("Failed to open stdin");
		let stderr = child.stderr.take().expect("Failed to open stderr");
		let mut obj = Self {
			child,
			_stdin,
			stdout,
			stderr,
		};
		obj.assert_not_failed();
		obj
	}

	pub fn open_client(_port: TcpPort) -> Self {
		// let mut builder = Command::new("nc");
		// builder.arg("-N")
		// 	.arg("127.0.0.1")
		// 	.arg(format!("{port}"))
		// 	.stdin(Stdio::piped())
		// 	.stdout(Stdio::piped());
		// let child = builder.spawn();
		// Self { port }
		todo!()
	}
}
