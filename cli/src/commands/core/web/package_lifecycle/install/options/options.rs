//! Typed input shared by the Web install flow owners.
//! Input có kiểu dùng chung trong các owner luồng cài Web.

/// Immutable CLI choices for one Web install invocation.
/// Các lựa chọn CLI bất biến của một lần cài Web.
pub(in crate::commands::core::web::implementation::package_lifecycle::install) struct WebInstallOptions
{
    pub(in crate::commands::core::web::implementation::package_lifecycle::install) packages:
        Vec<String>,
    pub(in crate::commands::core::web::implementation::package_lifecycle::install) frozen: bool,
    pub(in crate::commands::core::web::implementation::package_lifecycle::install) ignore_scripts:
        bool,
    pub(in crate::commands::core::web::implementation::package_lifecycle::install) allow_scripts:
        bool,
    pub(in crate::commands::core::web::implementation::package_lifecycle::install) prefer_dedupe:
        bool,
    pub(in crate::commands::core::web::implementation::package_lifecycle::install) repair: bool,
    pub(in crate::commands::core::web::implementation::package_lifecycle::install) offline: bool,
    pub(in crate::commands::core::web::implementation::package_lifecycle::install) compat_runtime:
        Option<String>,
}

impl WebInstallOptions {
    #[allow(clippy::too_many_arguments)] // Mirrors the explicit CLI install switches in one immutable value.
    pub(in crate::commands::core::web::implementation::package_lifecycle::install) fn new(
        packages: Vec<String>,
        frozen: bool,
        ignore_scripts: bool,
        allow_scripts: bool,
        prefer_dedupe: bool,
        repair: bool,
        offline: bool,
        compat_runtime: Option<String>,
    ) -> Self {
        Self {
            packages,
            frozen,
            ignore_scripts,
            allow_scripts,
            prefer_dedupe,
            repair,
            offline,
            compat_runtime,
        }
    }
}
