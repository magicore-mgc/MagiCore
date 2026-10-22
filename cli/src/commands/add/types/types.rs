//! Typed pass-through for the existing shared add arguments.
//! Gói các đối số add dùng chung hiện tại thành type nội bộ.

/// Existing add arguments passed unchanged to a selected core lane.
/// Các đối số add hiện có được chuyển nguyên trạng tới lane core đã chọn.
pub(in crate::commands::add) struct AddRequest {
    pub(in crate::commands::add) packages: Vec<String>,
    pub(in crate::commands::add) version: Option<String>,
    pub(in crate::commands::add) dev: bool,
    pub(in crate::commands::add) exact: bool,
    pub(in crate::commands::add) optional: bool,
    pub(in crate::commands::add) peer: bool,
    pub(in crate::commands::add) no_save: bool,
    pub(in crate::commands::add) global: bool,
    pub(in crate::commands::add) compat_runtime: Option<String>,
}
