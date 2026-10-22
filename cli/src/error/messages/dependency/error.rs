//! Dependency graph and package inspection diagnostics.
//! Lỗi đồ thị dependency và tra cứu package.

use anyhow::Error;
use anyhow::anyhow;

pub fn link_web_only() -> Error {
    anyhow!("link only supported for web core")
}

pub fn link_usage() -> Error {
    anyhow!("Usage: mgc link <package>")
}

pub fn link_exists(name: &str) -> Error {
    anyhow!("{name} already exists in node_modules")
}

pub fn unlink_web_only() -> Error {
    anyhow!("unlink only supported for web core")
}

pub fn unlink_usage() -> Error {
    anyhow!("Usage: mgc unlink <package>")
}

pub fn unlink_not_linked(name: &str) -> Error {
    anyhow!("{name} is not linked in node_modules")
}

pub fn why_web_only() -> Error {
    anyhow!("why only supported for web core")
}

pub fn local_path_not_found(path: &std::path::Path) -> Error {
    anyhow!("local package path not found: {}", path.display())
}

pub fn info_no_web_adapter() -> Error {
    anyhow!(
        "package info currently requires the web core registry adapter, which is not included in this build"
    )
}

pub fn outdated_no_web_adapter() -> Error {
    anyhow!(
        "outdated for the web core requires the web registry adapter, which is not included in this build"
    )
}

pub fn outdated_lock_unusable(core: &str) -> Error {
    anyhow!(
        "`mgc outdated` for core `{core}` requires a valid mgc.lock matching the current manifest; run `mgc install` to create or refresh it"
    )
}
