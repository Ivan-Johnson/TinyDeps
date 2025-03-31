#![forbid(unsafe_code)]
#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

#[cfg(feature = "daemon")]
pub mod daemon;
#[cfg(feature = "ipc")]
pub mod ipc;
