//! Wires the `version` owner and its nearest tests.
//! Nối owner `version` và test gần nhất.

mod version;
pub use self::version::Version;
#[cfg(test)]
mod test;
