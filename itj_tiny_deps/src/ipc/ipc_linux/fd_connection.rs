use crate::ipc::Connection;
use libc::c_int;
use libc::c_void;
use libc::close;
use libc::ssize_t;
use std::io::Error;

/// IIRC Linux guarantees that writes smaller than 4k are atomic; this size was
/// choosen accordingly.
const MAX_WRITE_SIZE: usize = 4_000;

pub struct FDConnection {
	pub(super) fd: c_int,
}

impl Drop for FDConnection {
	fn drop(&mut self) {
		unsafe { close(self.fd) };
	}
}

impl Connection for FDConnection {
	fn read(&mut self) -> Result<Vec<u8>, Error> {
		let mut buffer: Vec<u8> = vec![0; MAX_WRITE_SIZE];
		let ptr: *mut u8 = buffer.as_mut_ptr();
		let size: ssize_t = unsafe { libc::read(self.fd, ptr as *mut c_void, MAX_WRITE_SIZE) };
		if size == -1 {
			return Err(std::io::Error::last_os_error());
		}
		buffer.truncate(size.try_into().unwrap());
		Ok(buffer)
	}

	fn send(&mut self, msg: &[u8]) -> Result<(), Error> {
		let ptr: *const u8 = msg.as_ptr();
		let ptr = ptr as *const c_void;

		let size: ssize_t = unsafe { libc::write(self.fd, ptr, msg.len()) };
		if size == -1 {
			return Err(std::io::Error::last_os_error());
		}
		assert_eq!(msg.len(), size.try_into().unwrap());
		Ok(())
	}
}
