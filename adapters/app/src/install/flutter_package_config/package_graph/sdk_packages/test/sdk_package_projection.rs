//! Verify SDK package identity, version and supported dependency projection.
//! Kiểm tra identity, version và projection dependency SDK được hỗ trợ.

use super::*;

#[test]
fn sdk_package_projection_keeps_flutter_and_skips_dart_sdk_edges() {
    let sdk = tempfile::tempdir().unwrap();
    let package_root = sdk.path().join("packages/flutter");
    std::fs::create_dir_all(&package_root).unwrap();
    std::fs::write(
        package_root.join("pubspec.yaml"),
        concat!(
            "name: flutter\nversion: 3.24.0\ndependencies:\n",
            "  sky_engine:\n    sdk: flutter\n  collection: ^1.0.0\n",
            "  dart_internal:\n    sdk: dart\n"
        ),
    )
    .unwrap();

    let entries = super::read_sdk_package_entries(
        sdk.path(),
        &["flutter".to_string()],
        "sample",
        &BTreeSet::new(),
    )
    .unwrap();

    assert_eq!(entries["flutter"].version, "3.24.0");
    assert_eq!(
        entries["flutter"].dependencies,
        vec!["collection".to_string(), "sky_engine".to_string()]
    );
}
