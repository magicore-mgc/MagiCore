// Shared list mapping module — export the request and core conversion.
// Module map list dùng chung — xuất request và phép đổi sang core.
mod list_mapping;

pub(crate) use list_mapping::{ListRequest, ListTarget, to_core_command};
