//! Local package-declaration lookup exports.
//! Xuất chức năng tìm khai báo package trong project.

mod local_version;
pub(super) use local_version::detect_local_version;
