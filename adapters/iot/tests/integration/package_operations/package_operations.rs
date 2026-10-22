//! Package Operations behavior tests.
//! Test hành vi cho nhóm package operations.

use crate::integration::support::tmp;
use mgc_iot_adapter::adapter_for;
use mgc_types::PackageName;
use mgc_types::adapter::{AddOptions, PackageAdapter};
use mgc_types::capabilities::DependencyResolver;

pub async fn platformio_add_fails_closed_outside_explicit_cli_compatibility() {
    let dir = tmp("add-pio");
    std::fs::write(dir.join("platformio.ini"), "[env:esp32]\n").unwrap();
    let a = adapter_for(&dir).unwrap();
    let name = PackageName::new("arduino-json").unwrap();
    let error = a
        .add(&dir, &name, None, AddOptions::default())
        .await
        .unwrap_err();
    assert!(matches!(
        error,
        mgc_types::MgError::Unsupported {
            core: "iot",
            capability: "add",
            ..
        }
    ));
}

pub async fn zephyr_add_fails_closed_without_native_dependency_support() {
    let dir = tmp("add-zephyr");
    std::fs::write(dir.join("west.yml"), "manifest:\n").unwrap();
    let a = adapter_for(&dir).unwrap();
    let name = PackageName::new("lvgl").unwrap();
    let err = a
        .add(&dir, &name, None, AddOptions::default())
        .await
        .unwrap_err();
    let msg = err.to_string();
    assert!(
        msg.contains("MagiCore-owned") || msg.contains("unsupported"),
        "error must explain that adapter-level add is not native: {msg}"
    );
}

pub async fn zephyr_list_fails_closed_instead_of_returning_an_empty_native_list() {
    let dir = tmp("list-zephyr");
    std::fs::write(dir.join("west.yml"), "manifest:\n  projects: []\n").unwrap();
    let adapter = adapter_for(&dir).unwrap();
    let error = adapter.list(&dir).await.unwrap_err();
    assert!(matches!(
        error,
        mgc_types::MgError::Unsupported {
            core: "iot",
            capability: "list",
            ..
        }
    ));
    assert!(
        !adapter
            .capabilities()
            .contains(&mgc_types::capabilities::Capability::ContentStoreProvider)
    );
    assert!(mgc_types::capabilities::ContentStoreProvider::probe_content_store(&adapter).is_err());
}

pub async fn platformio_list_does_not_report_declared_ranges_as_installed_versions() {
    let dir = tmp("list-pio");
    std::fs::write(
        dir.join("platformio.ini"),
        "[env:board]\nlib_deps = bblanchon/ArduinoJson@^6\n",
    )
    .unwrap();
    let adapter = adapter_for(&dir).unwrap();
    let error = adapter.list(&dir).await.unwrap_err();
    assert!(matches!(
        error,
        mgc_types::MgError::Unsupported {
            capability: "list",
            ..
        }
    ));
}

pub async fn platformio_adapter_install_fails_closed_without_spawning_pio() {
    let dir = tmp("adapter-no-delegate");
    std::fs::write(dir.join("platformio.ini"), "[env:board]\n").unwrap();
    let adapter = adapter_for(&dir).unwrap();
    let graph = mgc_types::ResolvedGraph::empty();
    let error = mgc_types::capabilities::ContentStoreProvider::install(
        &adapter,
        &graph,
        &dir,
        mgc_types::adapter::InstallOptions::default(),
    )
    .await
    .unwrap_err();
    assert!(matches!(error, mgc_types::MgError::Unsupported { .. }));
}
