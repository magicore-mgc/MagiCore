//! Verify unsupported SDK dependency sources are not silently omitted.
//! Kiểm tra source dependency SDK chưa hỗ trợ không bị bỏ âm thầm.

use super::*;

#[test]
fn unsupported_sdk_dependency_source_returns_typed_error() {
    let manifest: serde_yaml::Value = serde_yaml::from_str(
        "dependencies:\n  local_package:\n    git: https://example.invalid/repo.git\n",
    )
    .unwrap();

    let error =
        super::super::sdk_dependencies::sdk_dependency_names("flutter", &manifest).unwrap_err();

    assert!(matches!(error, FlutterPackageGraphError::Shared(_)));
}
