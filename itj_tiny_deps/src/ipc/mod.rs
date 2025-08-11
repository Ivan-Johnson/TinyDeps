#[cfg(feature = "ipc_linux")]
pub mod ipc_linux;
mod ipc_message;
mod traits;

pub use ipc_message::Message;
pub use ipc_message::MessageConnection;
pub use traits::Connection;
pub use traits::Server;
pub use traits::TcpPort;
