//! Shared immutable input for runtime and framework launch owners.
//! Input bất biến dùng chung giữa owner runtime và framework.

use std::ffi::OsString;
use std::path::Path;

/// Resolved package-script context; visibility stays inside package launch.
/// Context package script đã resolve; visibility chỉ trong package launch.
pub(in crate::commands::core::web::implementation::dev::package_launch) struct LaunchContext<'a> {
    pub(in crate::commands::core::web::implementation::dev::package_launch) project_root: &'a Path,
    pub(in crate::commands::core::web::implementation::dev::package_launch) tokens: &'a [&'a str],
    pub(in crate::commands::core::web::implementation::dev::package_launch) host: Option<&'a str>,
    pub(in crate::commands::core::web::implementation::dev::package_launch) port: Option<u16>,
    pub(in crate::commands::core::web::implementation::dev::package_launch) base_envs:
        &'a [(OsString, OsString)],
}
