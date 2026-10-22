//! Stable native engine API facade.
//! Facade API ổn định cho native engine.

mod engine;

#[cfg(test)]
mod test;

pub use engine::{NativeResolution, resolve_with_protocol, resolve_with_protocol_including_dev};
