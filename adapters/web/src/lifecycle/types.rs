//! Lifecycle manifest data used by policy inspection and execution.
//! Kiểu dữ liệu manifest lifecycle dùng chung cho policy và thực thi.

use serde::Deserialize;

#[derive(Debug, Deserialize, Default, Clone)]
pub(crate) struct PackageScripts {
    pub(crate) preinstall: Option<String>,
    pub(crate) install: Option<String>,
    pub(crate) postinstall: Option<String>,
}

impl PackageScripts {
    pub(crate) fn has_hooks(&self) -> bool {
        self.preinstall.is_some() || self.install.is_some() || self.postinstall.is_some()
    }
}

#[derive(Debug, Deserialize)]
pub(super) struct PackageManifest {
    #[serde(default)]
    pub(super) scripts: PackageScripts,
}
