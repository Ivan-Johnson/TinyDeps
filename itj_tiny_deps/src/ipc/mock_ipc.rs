use crate::ipc::Connection;
use crate::ipc::Server;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::io::Error;
use std::io::ErrorKind;
use std::rc::Rc;

/// One queued result for a future `MockConnection::read()` call.
#[derive(Debug, Clone)]
enum MockRead {
	/// A complete message's bytes, as `Connection::read` would return them.
	Message(Vec<u8>),
	/// An EOF: `Connection::read` returning an empty buffer.
	Eof,
	/// No data available yet: `Connection::read` returning `ErrorKind::WouldBlock`.
	WouldBlock,
}

#[derive(Debug, Default)]
struct MockConnectionData {
	reads: VecDeque<MockRead>,
	sent: Vec<Vec<u8>>,
}

/// An in-memory test double for `ipc::Connection`.
///
/// Cloning shares the same underlying state (like `MockNotification`), so a test can hand one
/// clone to a `Server`/`RpcServer` under test while keeping another clone to queue reads and
/// inspect what was sent.
#[derive(Debug, Default, Clone)]
pub struct MockConnection {
	data: Rc<RefCell<MockConnectionData>>,
}

impl MockConnection {
	#[must_use]
	pub fn new() -> Self {
		Self::default()
	}

	/// Queue a message to be returned by a future `read()` call.
	pub fn push_message(&self, bytes: Vec<u8>) {
		self.data
			.borrow_mut()
			.reads
			.push_back(MockRead::Message(bytes));
	}

	/// Queue an EOF to be returned by a future `read()` call.
	pub fn push_eof(&self) {
		self.data.borrow_mut().reads.push_back(MockRead::Eof);
	}

	/// Queue a `WouldBlock` error to be returned by a future `read()` call.
	pub fn push_would_block(&self) {
		self.data.borrow_mut().reads.push_back(MockRead::WouldBlock);
	}

	/// The bytes passed to every `send()` call so far, in order.
	#[must_use]
	pub fn sent(&self) -> Vec<Vec<u8>> {
		self.data.borrow().sent.clone()
	}
}

impl Connection for MockConnection {
	fn read(&mut self) -> Result<Vec<u8>, Error> {
		let queued = self
			.data
			.borrow_mut()
			.reads
			.pop_front()
			.expect("MockConnection.read() called more times than a result was queued for");
		match queued {
			MockRead::Message(bytes) => Ok(bytes),
			MockRead::Eof => Ok(Vec::new()),
			MockRead::WouldBlock => Err(Error::from(ErrorKind::WouldBlock)),
		}
	}

	fn send(&mut self, msg: &[u8]) -> Result<(), Error> {
		self.data.borrow_mut().sent.push(msg.to_vec());
		Ok(())
	}
}

/// An in-memory test double for `ipc::Server<MockConnection>`.
#[derive(Debug, Default)]
pub struct MockServer {
	pending: VecDeque<MockConnection>,
}

impl MockServer {
	#[must_use]
	pub fn new() -> Self {
		Self::default()
	}

	/// Queue a connection to be returned by a future `poll_connection()` call.
	pub fn enqueue_connection(&mut self, connection: MockConnection) {
		self.pending.push_back(connection);
	}
}

impl Server<MockConnection> for MockServer {
	fn poll_connection(&mut self) -> Option<MockConnection> {
		self.pending.pop_front()
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_connection_read_returns_queued_message() {
		let mut conn = MockConnection::new();
		conn.push_message(vec![1, 2, 3]);
		assert_eq!(vec![1, 2, 3], conn.read().unwrap());
	}

	#[test]
	fn test_connection_read_returns_eof_as_empty() {
		let mut conn = MockConnection::new();
		conn.push_eof();
		assert_eq!(Vec::<u8>::new(), conn.read().unwrap());
	}

	#[test]
	fn test_connection_read_returns_would_block() {
		let mut conn = MockConnection::new();
		conn.push_would_block();
		assert_eq!(ErrorKind::WouldBlock, conn.read().unwrap_err().kind());
	}

	#[test]
	fn test_connection_send_is_recorded() {
		let mut conn = MockConnection::new();
		conn.send(&[9, 8, 7]).unwrap();
		assert_eq!(vec![vec![9, 8, 7]], conn.sent());
	}

	#[test]
	fn test_connection_clone_shares_state() {
		let conn = MockConnection::new();
		let clone = conn.clone();
		clone.push_message(vec![42]);
		let mut conn = conn;
		assert_eq!(vec![42], conn.read().unwrap());
	}

	#[test]
	#[should_panic(expected = "read() called more times")]
	fn test_connection_read_without_queued_result_panics() {
		let mut conn = MockConnection::new();
		let _ = conn.read();
	}

	#[test]
	fn test_server_poll_connection_returns_queued_connections_in_order() {
		let mut server = MockServer::new();
		let first = MockConnection::new();
		let second = MockConnection::new();
		server.enqueue_connection(first.clone());
		server.enqueue_connection(second.clone());

		first.push_message(vec![1]);
		second.push_message(vec![2]);

		let mut polled_first = server.poll_connection().unwrap();
		let mut polled_second = server.poll_connection().unwrap();
		assert_eq!(vec![1], polled_first.read().unwrap());
		assert_eq!(vec![2], polled_second.read().unwrap());
	}

	#[test]
	fn test_server_poll_connection_returns_none_when_empty() {
		let mut server = MockServer::new();
		assert!(server.poll_connection().is_none());
	}
}
