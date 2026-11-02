//! Wire the Windows system snapshot test seam.
//! Nối test seam chụp process table Windows.

#[path = "system_snapshot.rs"]
mod system_snapshot;

// The snapshot tests drive the inspector directly and build an ancestry value,
// so both are forwarded to `run::implementation` for them to name.
// (Test snapshot gọi inspector trực tiếp và dựng giá trị ancestry, nên cả hai
// được chuyển tiếp tới `run::implementation` để test gọi tên.)
//
// Re-exporting the imports of `system_snapshot.rs` failed with E0603: a plain
// `use` binding is not a re-exportable item, and this module owns neither the
// ancestry type nor the inspector signature. `super::ancestry` owns and
// publishes the type; the inspector is owned by the child module declared
// above, so it is re-exported with `self::`, never through the sibling import
// list in `system_snapshot.rs` (that name only lives in the child's scope).
// (Re-export các import của `system_snapshot.rs` lỗi E0603: binding `use` trần
// không phải item re-export được, và module này không sở hữu type ancestry lẫn
// chữ ký inspector. `super::ancestry` sở hữu và công khai type; inspector thuộc
// module con khai báo ở trên, nên re-export bằng `self::`, không qua danh sách
// import anh em trong `system_snapshot.rs` (tên đó chỉ sống trong scope con).)
#[cfg(all(windows, test))]
pub(in crate::run::implementation) use self::system_snapshot::find_forbidden_descendant_with_timeout;
#[cfg(all(windows, test))]
pub(in crate::run::implementation) use super::ancestry::WindowsProcessAncestry;
