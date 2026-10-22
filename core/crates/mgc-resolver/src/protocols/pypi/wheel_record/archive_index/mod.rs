//! Wiring facade for authenticated wheel archive indexing.
//! Facade wiring cho việc index archive wheel đã xác thực.

#[path = "archive_index.rs"]
mod implementation;

pub(super) use implementation::{ArchiveIndex, index_wheel_entries};
