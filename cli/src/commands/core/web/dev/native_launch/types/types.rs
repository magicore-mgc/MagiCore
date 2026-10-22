//! Shared project and endpoint input for language-specific native launchers.
//! Input project và endpoint dùng chung cho launcher native từng ngôn ngữ.

use std::path::Path;

/// Detected project root and user-selected endpoint.
/// Project root đã nhận diện và endpoint do người dùng chọn.
pub(in crate::commands::core::web::implementation::dev::native_launch) struct NativeDevContext<'a> {
    pub(in crate::commands::core::web::implementation::dev::native_launch) project_root: &'a Path,
    pub(in crate::commands::core::web::implementation::dev::native_launch) host: Option<&'a str>,
    pub(in crate::commands::core::web::implementation::dev::native_launch) port: Option<u16>,
}
