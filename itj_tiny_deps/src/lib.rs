#![forbid(unsafe_code)]
#![cfg_attr(debug_assertions, allow(dead_code, unused_imports, unreachable_code))]

#[cfg(feature = "daemon")]
pub mod daemon;
#[cfg(feature = "error_handling")]
pub mod errors;
#[cfg(feature = "ipc")]
pub mod ipc;
#[cfg(feature = "random")]
pub mod random;
