mod child_kill_on_drop;
mod from_file;
mod join_handle_join_on_drop;
mod random_trait;
mod sway_subscribe;

pub use from_file::RandomFromFile;
pub use random_trait::Random;

// TODO: This doesn't belong in the "random" module. Random means random, not miscellaneous.
pub use sway_subscribe::SwayInput;
pub use sway_subscribe::SwayInputEvent;
pub use sway_subscribe::SwayInputSubscribe;

// TODO: I just found the [oorandom](https://lib.rs/crates/oorandom) crate. It's
// genuinely tiny, has no dependencies, and seems to do exactly what I want. Add
// a new implementation of my trait that's a wrapper around it? Gate behind a
// feature flag, obviously. Possibly even move to an `itj_tiny_deps_3p` crate?
