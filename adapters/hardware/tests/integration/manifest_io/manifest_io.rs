//! Manifest Io behavior tests.
//! Checks Hardware manifest parsing and project-name derivation.

use crate::integration::support::tmp;
use mgc_hardware_adapter::HardwareAdapter;
use mgc_types::adapter::PackageAdapter;

pub async fn parse_manifest_uses_dir_name() {
    let dir = tmp("my-hardware-project");
    std::fs::write(dir.join("mgc.toml"), "ecosystem = \"hardware\"\n").unwrap();
    let manifest = HardwareAdapter.parse_manifest(&dir).await.unwrap();
    assert!(manifest.name.contains("my-hardware-project"));
}
