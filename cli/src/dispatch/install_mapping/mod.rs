// Shared install mapping module — export normalized request and core conversion.
// Module map install dùng chung — xuất request chuẩn hóa và phép đổi sang core.
mod install_mapping;

pub(crate) use install_mapping::{InstallRequest, InstallTarget, to_core_command};
