#[cfg(feature = "ipc_linux_fd_connection")]
mod fd_connection;
#[cfg(feature = "ipc_linux_inet_client")]
mod inet_client;
#[cfg(feature = "ipc_linux_inet_server")]
mod inet_server;
mod sockaddr;

/// Implement `ipc::Connection` using a Linux file descriptor
pub use fd_connection::FDConnection;

/// Implement `ipc::Connection` using Linux's `AF_INET`
/// <https://man.archlinux.org/man/ip.7.en>
#[cfg(feature = "ipc_linux_inet_server")]
pub use inet_server::InetServer;

/// Implement `ipc::Connection` using Linux's `AF_INET`
/// <https://man.archlinux.org/man/ip.7.en>
#[cfg(feature = "ipc_linux_inet_client")]
pub use inet_client::new_inet_client;
