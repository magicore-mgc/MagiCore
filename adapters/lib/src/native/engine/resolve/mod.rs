//! Resolve owner wiring.
//! Nối owner resolve.

mod resolve;

#[cfg(test)]
mod test;

pub use resolve::{resolve_with_protocol, resolve_with_protocol_including_dev};
