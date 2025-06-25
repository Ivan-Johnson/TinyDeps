// Bizarrely, `forbid(unsafe_code)` doesn't forbid us from declaring a macro
// that uses `unsafe`, nor does it even forbid is from using a macro that uses
// `unsafe`.
#![forbid(unsafe_code)]
#![deny(clippy::large_stack_frames)]
#![cfg_attr(debug_assertions, allow(dead_code, unused_imports, unreachable_code))]

#[cfg(feature = "boxed_array")]
pub mod boxed_array;
#[cfg(feature = "command")]
pub mod command;
#[cfg(feature = "daemon")]
pub mod daemon;
#[cfg(feature = "error_handling")]
pub mod errors;
#[cfg(feature = "http")]
pub mod http;
#[cfg(feature = "ipc")]
pub mod ipc;
#[cfg(feature = "random")]
pub mod random;
#[cfg(feature = "time")]
pub mod time;
