//! Component display name mapping — Ánh xạ tên hiển thị component

use crate::mapping::package_purl::swift_purl_path;
use mgc_lockfile::EcosystemTag;

pub(crate) fn component_name(ecosystem: EcosystemTag, name: &str) -> String {
    if ecosystem == EcosystemTag::Swift {
        return swift_purl_path(name)
            .and_then(|path| path.rsplit('/').next().map(str::to_string))
            .filter(|name| !name.is_empty())
            .unwrap_or_else(|| "swift-package".to_string());
    }
    name.to_string()
}
#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
