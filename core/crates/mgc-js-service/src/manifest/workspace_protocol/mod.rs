//! Workspace protocol recognition exports.
//! Export nhận diện workspace protocol.

mod workspace_protocol;

pub use workspace_protocol::is_workspace_protocol_range;

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
