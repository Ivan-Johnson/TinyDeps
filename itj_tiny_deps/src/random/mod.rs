mod from_file;
mod random_trait;

pub use from_file::RandomFromFile;
pub use random_trait::Random;

// TODO: I just found the [oorandom](https://lib.rs/crates/oorandom) crate. It's
// genuinely tiny, has no dependencies, and seems to do exactly what I want. Add
// a new implementation of my trait that's a wrapper around it? Gate behind a
// feature flag, obviously. Possibly even move to an `itj_tiny_deps_3p` crate?
