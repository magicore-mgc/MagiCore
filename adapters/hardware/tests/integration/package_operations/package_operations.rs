//! Package Operations behavior tests.
//! Verifies unsupported package mutations and inventory fail closed.

use crate::integration::support::tmp;
use mgc_hardware_adapter::HardwareAdapter;
use mgc_types::adapter::PackageAdapter;
use mgc_types::capabilities::DependencyResolver;

pub async fn adapter_package_list_rejects_template_inventory() {
    let dir = tmp("list-empty");
    std::fs::write(dir.join("mgc.toml"), "ecosystem = \"hardware\"\n").unwrap();
    std::fs::create_dir_all(dir.join("optimizer")).unwrap();
    let error = HardwareAdapter.list(&dir).await.unwrap_err();
    assert!(matches!(
        error,
        mgc_types::MgError::Unsupported {
            capability: "list",
            ..
        }
    ));
}

pub async fn add_fails_for_hardware_adapter() {
    use mgc_types::PackageName;
    use mgc_types::adapter::AddOptions;
    let dir = tmp("add-fail");
    std::fs::write(dir.join("mgc.toml"), "ecosystem = \"hardware\"\n").unwrap();
    let name = PackageName::new("hal-crate").unwrap();
    let result = HardwareAdapter
        .add(&dir, &name, None, AddOptions::default())
        .await;
    assert!(result.is_err(), "hardware adapter must block direct add");
}

pub async fn remove_fails_for_hardware_adapter() {
    use mgc_types::PackageName;
    let dir = tmp("rem-fail");
    std::fs::write(dir.join("mgc.toml"), "ecosystem = \"hardware\"\n").unwrap();
    let name = PackageName::new("hal-crate").unwrap();
    let result = HardwareAdapter.remove(&dir, &name).await;
    assert!(result.is_err(), "hardware adapter must block direct remove");
}
