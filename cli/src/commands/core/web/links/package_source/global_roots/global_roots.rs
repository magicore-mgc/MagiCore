//! Resolve configured and default global Web package roots in search order.
//! Tạo danh sách root package Web global theo thứ tự tìm kiếm hiện hành.

use std::path::PathBuf;

pub(in crate::commands::core::web::links::package_source) fn magicore_global_package_roots()
-> Vec<PathBuf> {
    let override_path = std::env::var_os("MAGICORE_GLOBAL_PACKAGE_ROOT").map(PathBuf::from);
    global_package_roots(
        override_path,
        mgc_platform::paths::mgc_data_dir(),
        dirs::cache_dir(),
    )
}

pub(super) fn global_package_roots(
    override_path: Option<PathBuf>,
    data_dir: PathBuf,
    cache_dir: Option<PathBuf>,
) -> Vec<PathBuf> {
    let mut roots = Vec::new();
    if let Some(path) = override_path {
        roots.push(path);
    }
    roots.push(
        data_dir
            .join("cache")
            .join("web")
            .join("global")
            .join("node_modules"),
    );
    if let Some(cache_dir) = cache_dir {
        roots.push(
            cache_dir
                .join("magicore")
                .join("global")
                .join("web")
                .join("node_modules"),
        );
    }
    roots
}
