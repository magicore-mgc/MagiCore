#![cfg(test)]
#![allow(clippy::unwrap_used)]

//! MagiCore install-root ownership tests — test quyền sở hữu gốc cài MagiCore.

use mgc_lib_adapter::install::shared_store::SharedStoreRun;

#[test]
fn native_python_install_root_is_under_mgc_store() {
    let first = SharedStoreRun::pypi().unwrap();
    let second = SharedStoreRun::pypi().unwrap();
    // Expect the centralized data root (MGC_DATA_DIR or ~/MagiCore-SystemData).
    // Kỳ vọng root dữ liệu tập trung (MGC_DATA_DIR hoặc ~/MagiCore-SystemData).
    let expected = mgc_platform::paths::GlobalPaths::new()
        .unwrap()
        .store
        .join("pypi");

    assert_eq!(first.install_root, expected);
    assert_eq!(second.install_root, expected);
    assert_eq!(first.install_root, second.install_root);
}
