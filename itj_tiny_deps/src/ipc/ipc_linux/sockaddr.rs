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

pub fn in_addr_from_ipv4addr(addr: Ipv4Addr) -> libc::in_addr {
	let addr: u32 /* host endianness */ = addr.to_bits();
	let addr: u32 /* ipv4 endianness */ = network_u32_from_host(addr);
	libc::in_addr { s_addr: addr }
}

pub fn new_sockaddr_in(addr: Ipv4Addr, port: TcpPort) -> sockaddr_in {
	sockaddr_in {
		sin_family: AF_INET.try_into().unwrap(),
		sin_addr: in_addr_from_ipv4addr(addr),
		sin_port: network_u16_from_host(port),

		// This struct requires padding
		sin_zero: [0; 8],
	}
}
