use crate::ipc::ipc_linux::fd_connection::FDConnection;
use crate::ipc::ipc_linux::sockaddr::new_sockaddr_in;
use crate::ipc::TcpPort;
use libc::connect;
use libc::sockaddr_in;
use libc::socket;
use libc::socklen_t;
use libc::AF_INET;
use libc::IPPROTO_TCP;
use libc::SOCK_STREAM;
use std::io::Error;
use std::net::Ipv4Addr;

pub fn new_inet_client(port: TcpPort) -> FDConnection {
	// strace nc -N 127.0.0.1 7320

	// 1. Trivially create the socket
	// Ideally I'd use `SOCK_NONBLOCK`, but idc.
	let fd = unsafe { socket(AF_INET, SOCK_STREAM, IPPROTO_TCP) };
	assert_ne!(fd, -1);

	// 2. Connect to the server
	let addr_size: socklen_t = std::mem::size_of::<sockaddr_in>().try_into().unwrap();
	let addr = new_sockaddr_in(Ipv4Addr::new(127, 0, 0, 1), port);
	let addr_ptr = &addr as *const sockaddr_in as *const libc::sockaddr;

	let ret_connect = unsafe { connect(fd, addr_ptr, addr_size) };
	let err = Error::last_os_error();
	assert_eq!(
		ret_connect, 0,
		"Failed to connect to port {port}. errno {err}"
	);
	FDConnection { fd }
}
