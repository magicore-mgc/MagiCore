// Shared update mapping exports — keep command construction behind one owner.
// Export mapping update dùng chung — gom khâu dựng lệnh tại một owner.
mod update_mapping;

pub(crate) use update_mapping::{UpdateRequest, UpdateTarget, to_core_command};
