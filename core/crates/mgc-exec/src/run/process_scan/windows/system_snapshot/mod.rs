//! Wire the Windows system snapshot test seam.
//! Nối test seam chụp process table Windows.

#[path = "system_snapshot.rs"]
mod system_snapshot;

// The snapshot tests drive the inspector directly and build an ancestry value,
// so both the function and the type are forwarded where the tests can name them.
// (Test snapshot gọi inspector trực tiếp và dựng giá trị ancestry, nên cả hàm
// và type được chuyển tiếp tới nơi test gọi được.)
#[cfg(all(windows, test))]
pub(in crate::run::implementation) use super::implementation::WindowsProcessAncestry;
#[cfg(all(windows, test))]
pub(in crate::run::implementation) use system_snapshot::find_forbidden_descendant_with_timeout;
