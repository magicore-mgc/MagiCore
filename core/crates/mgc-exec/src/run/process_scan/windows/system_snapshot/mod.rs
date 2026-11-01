//! Wire the Windows system snapshot test seam.
//! Nối test seam chụp process table Windows.

#[path = "system_snapshot.rs"]
mod system_snapshot;

// The snapshot tests drive the inspector directly and build an ancestry value,
// so both the function and the type are forwarded where the tests can name them
// through `super::from` the child `test` module.
// (Test snapshot gọi inspector trực tiếp và dựng giá trị ancestry, nên cả hàm
// và type được chuyển tiếp tới `super` của module test con.)
//
// The ancestry type has no local owner here, so the facade re-exports it from
// the `system_snapshot` module's own imports rather than reaching for a
// sibling `implementation` module, which does not exist.
// (Type ancestry không có owner cục bộ ở đây, nên facade re-export lại từ chính
// import của module `system_snapshot`, không trỏ tới module `implementation`
// anh em vì module đó không tồn tại.)
#[cfg(all(windows, test))]
use system_snapshot::WindowsProcessAncestry;
#[cfg(all(windows, test))]
pub(in crate::run::implementation) use system_snapshot::find_forbidden_descendant_with_timeout;
