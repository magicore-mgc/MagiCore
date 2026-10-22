//! Cargo.toml reading and single-package subset validation.
//! Đọc Cargo.toml và kiểm tra tập con single-package được hỗ trợ.

mod project_manifest;
pub(in crate::install) use project_manifest::read_project_manifest;
