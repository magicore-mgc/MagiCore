//! Verify malformed dependency sections fail closed.
//! Kiểm tra section dependency sai phải fail-closed.

use super::*;

#[test]
fn dependency_name_projection_rejects_non_mapping_sections() {
    let pubspec: serde_yaml::Value = serde_yaml::from_str("dependencies: nope").unwrap();
    let error =
        super::super::flutter_pubspec_dependency_names(&pubspec, "dependencies").unwrap_err();

    assert!(matches!(
        error,
        FlutterPackageGraphError::RootDependencySectionNotMapping(section)
            if section == "dependencies"
    ));
}
