// Shared remove mapping exports — keep construction behind one owner.
// Export mapping remove dùng chung — gom khâu dựng lệnh tại một owner.
mod remove_mapping;

pub(crate) use remove_mapping::{RemoveRequest, RemoveTarget, to_core_command};
