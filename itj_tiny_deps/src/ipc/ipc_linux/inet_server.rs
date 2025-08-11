use crate::ipc::ipc_linux::fd_connection::FDConnection;
use crate::ipc::ipc_linux::sockaddr::new_sockaddr_in;
use crate::ipc::Connection;
use crate::ipc::Server;
use crate::ipc::TcpPort;
use libc::accept;
use libc::bind;
use libc::c_int;
use libc::c_void;
use libc::close;
use libc::connect;
use libc::htonl as network_u32_from_host;
use libc::htons as network_u16_from_host;
use libc::listen;
use libc::sockaddr;
use libc::sockaddr_in;
use libc::socket;
use libc::socklen_t;
use libc::ssize_t;
use libc::AF_INET;
use libc::EINPROGRESS;
use libc::IPPROTO_TCP;
use libc::SOCK_NONBLOCK;
use libc::SOCK_STREAM;
use std::io::Error;
use std::io::ErrorKind;
use std::net::Ipv4Addr;
use std::ptr::null_mut;

const MAX_PENDING_CONNECTIONS: c_int = 10;

#[derive(Debug)]
pub struct InetServer {
	fd: c_int,
}

impl Drop for InetServer {
	fn drop(&mut self) {
		unsafe { close(self.fd) };
	}
}

impl InetServer {
	pub fn new(port: TcpPort) -> Self {
		// 1. Trivially create the socket
		let fd = unsafe { socket(AF_INET, SOCK_STREAM | SOCK_NONBLOCK, IPPROTO_TCP) };
		assert_ne!(fd, -1);

		// 2. Bind the socket to a port and address
		//
		//    Note that the C function signature is a lie. It's `addr` argument has type
		//    `const struct sockaddr *`, when in reality it could be anything. I don't
		//    understand why they didn't just use a void pointer instead.
		//
		//    https://man.archlinux.org/man/bind.2.en#DESCRIPTION
		//
		//    > The actual structure passed for the addr argument will depend on the
		//    > address family.
		//
		//    For `AF_INET`, we use `sockaddr_in`: https://man.archlinux.org/man/ip.7.en#Address_format
		let addr_size: socklen_t = std::mem::size_of::<sockaddr_in>().try_into().unwrap();
		let addr = new_sockaddr_in(Ipv4Addr::new(127, 0, 0, 1), port);
		let addr_ptr = &addr as *const sockaddr_in as *const libc::sockaddr;

		let ret_bind = unsafe { bind(fd, addr_ptr, addr_size) };
		let err = Error::last_os_error();
		assert_eq!(
			ret_bind, 0,
			"Failed to bind server to port {port}. ret: {ret_bind}, errno: {err}"
		);

		// 3. Configure the socket as listening for connections
		let ret_listen = unsafe { listen(fd, MAX_PENDING_CONNECTIONS) };
		assert_eq!(ret_listen, 0);

		Self { fd }
	}
}

impl Server<FDConnection> for InetServer {
	fn poll_connection(&mut self) -> Option<FDConnection> {
		// If we wanted to, we could get the address of the client we
		// are connected to
		let addr_ptr: *mut sockaddr = null_mut();
		let size_ptr: *mut u32 = null_mut();

		let fd = unsafe { libc::accept4(self.fd, addr_ptr, size_ptr, SOCK_NONBLOCK) };

		if fd == -1 {
			// TODO: assert that `errno` is either `EAGAIN` or `EWOULDBLOCK`
			return None;
		}
		Some(FDConnection { fd })
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::ipc::ipc_linux::new_inet_client;
	use std::sync::atomic::AtomicUsize;
	use std::sync::atomic::Ordering;
	use std::time::Duration;

	const ITJ_TINY_DEPS_TEST_PORT: TcpPort = 6100;

	fn get_new_port() -> TcpPort {
		static COUNTER: AtomicUsize = AtomicUsize::new(0);
		let offset = COUNTER.fetch_add(1, Ordering::Relaxed);
		let offset: TcpPort = offset.try_into().unwrap();
		ITJ_TINY_DEPS_TEST_PORT + offset
	}

	#[test]
	fn test_server_create() {
		let port = get_new_port();
		let _server = InetServer::new(port);
	}

	#[test]
	#[should_panic]
	fn test_client_without_server() {
		let port = get_new_port();
		// This should fail because the server is not running
		let _client = new_inet_client(port);
	}

	#[test]
	#[should_panic]
	fn test_two_servers_one_port() {
		let port = get_new_port();
		let _server1 = InetServer::new(port);
		let _server2 = InetServer::new(port); // conflict should cause panic
	}

	#[test]
	fn test_server_no_connection() {
		let port = get_new_port();
		let mut server = InetServer::new(port);
		let con = server.poll_connection();
		assert!(con.is_none());
	}

	#[test]
	fn test_happy() {
		let port = get_new_port();
		let mut server = InetServer::new(port);
		std::thread::sleep(Duration::from_millis(200));
		let mut client_con = new_inet_client(port);
		let mut server_con = server.poll_connection().unwrap();

		// client -> server
		let data: Vec<u8> = vec![123];
		client_con.send(&data).unwrap();
		let response = server_con.read().unwrap();
		assert_eq!(data, response);

		// server -> client
		let data: Vec<u8> = vec![3, 2, 1];
		server_con.send(&data).unwrap();
		let response = client_con.read().unwrap();
		assert_eq!(data, response);
	}

	// TODO: write more tests
	//
	// ideas:
	// * two clients talking to the same server
	// * drop client connection, then server connection
	// * drop server connection, then client connection
	// *
}
