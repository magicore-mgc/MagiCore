// Shared add mapping module — export normalized request and core conversion.
// Module map add dùng chung — xuất request chuẩn hóa và phép đổi sang core.
mod add_mapping;

pub(crate) use add_mapping::{AddRequest, AddTarget, to_core_command};
