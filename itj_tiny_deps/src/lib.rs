#![cfg_attr(test, allow(clippy::large_stack_frames))]
#![cfg_attr(debug_assertions, allow(dead_code, unused_imports, unreachable_code))]

#[cfg(feature = "boxed_array")]
pub mod boxed_array;
#[cfg(feature = "command")]
pub mod command;
pub mod errors;
#[cfg(feature = "http")]
pub mod http;
#[cfg(feature = "ipc")]
pub mod ipc;
pub mod join_handle_join_on_drop;
#[cfg(feature = "notification")]
pub mod notification;
pub mod plugins;
#[cfg(feature = "random")]
pub mod random;
#[cfg(feature = "time")]
pub mod time;
