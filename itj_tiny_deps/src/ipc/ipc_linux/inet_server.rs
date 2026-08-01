use crate::ipc::ipc_linux::fd_connection::FDConnection;
use crate::ipc::ipc_linux::sockaddr::new_sockaddr_in;
use crate::ipc::Server;
use crate::ipc::TcpPort;
use libc::bind;
use libc::c_int;
use libc::close;
use libc::getsockname;
use libc::listen;
use libc::ntohs as host_u16_from_network;
use libc::sockaddr;
use libc::sockaddr_in;
use libc::socket;
use libc::socklen_t;
use libc::AF_INET;
use libc::IPPROTO_TCP;
use libc::SOCK_NONBLOCK;
use libc::SOCK_STREAM;
use std::io::Error;
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
	#[must_use]
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
		let addr = new_sockaddr_in(Ipv4Addr::LOCALHOST, port);
		let addr_ptr = (&raw const addr).cast::<libc::sockaddr>();

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

	#[must_use]
	pub fn get_port(&self) -> TcpPort {
		let size_const: socklen_t = std::mem::size_of::<sockaddr_in>().try_into().unwrap();
		let mut size_mut: socklen_t = size_const;
		let size_mut_ptr = &raw mut size_mut;

		let mut addr = new_sockaddr_in(Ipv4Addr::UNSPECIFIED, 0);
		let addr_ptr = (&raw mut addr).cast::<libc::sockaddr>();

		let ret_get = unsafe { getsockname(self.fd, addr_ptr, size_mut_ptr) };
		assert_eq!(0, ret_get);
		assert_eq!(size_const, size_mut);

		// println!("get_port -> {}", string_from_sockaddr_in(addr));
		host_u16_from_network(addr.sin_port)
	}
}

impl Default for InetServer {
	fn default() -> Self {
		Self::new(0)
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
	use crate::ipc::traits::Connection;
	use std::time::Duration;

	#[test]
	fn test_server_create() {
		InetServer::default();
	}

	#[test]
	#[should_panic]
	fn test_client_without_server() {
		let srv = InetServer::default();
		let port = srv.get_port();
		drop(srv);

		// This should fail because the server is not running
		new_inet_client(port);
	}

	#[test]
	#[should_panic]
	fn test_two_servers_one_port() {
		let server1 = InetServer::default();
		let port1 = server1.get_port();
		std::thread::sleep(Duration::from_millis(200));

		// Attempting to setup a second server on the same port should
		// cause a panic
		let server2 = InetServer::new(port1);
		let port2 = server2.get_port();
		println!("Test did not panic as expected?? {port1}, {port2}");
	}

	#[test]
	fn test_server_no_connection() {
		let mut server = InetServer::default();
		let con = server.poll_connection();
		assert!(con.is_none());
	}

	#[test]
	fn test_happy() {
		let mut server = InetServer::default();
		let port = server.get_port();
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
