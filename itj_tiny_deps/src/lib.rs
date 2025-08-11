#![deny(clippy::large_stack_frames)]
#![cfg_attr(debug_assertions, allow(dead_code, unused_imports, unreachable_code))]

#[cfg(feature = "boxed_array")]
pub mod boxed_array;
#[cfg(feature = "command")]
pub mod command;
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
