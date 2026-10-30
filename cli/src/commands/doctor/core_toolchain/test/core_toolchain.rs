//! Verifies core-to-toolchain mapping and core-tool requirement resolution.
//! Kiểm tra ánh xạ core→toolchain và việc giải yêu cầu tool theo core.

use super::super::core_toolchain::{CORE_REQUIRED_TOOLCHAINS, requirements_for_core};

#[test]
fn table_covers_all_nine_cores_in_canonical_form() {
    let cores: Vec<&str> = CORE_REQUIRED_TOOLCHAINS
        .iter()
        .map(|row| row.core)
        .collect();

    assert_eq!(
        cores,
        vec![
            "web", "lib", "ai", "app", "game", "iot", "clo", "cicd", "hardware"
        ]
    );
}

#[test]
fn web_core_requires_node_only() {
    let requirements = requirements_for_core("web");

    assert_eq!(requirements.len(), 1);
    assert_eq!(requirements[0].core, "web");
    assert_eq!(requirements[0].binary, "node");
    // Reuses the general probe arguments instead of a private copy.
    // Dùng lại đối số probe chung thay vì bản sao riêng.
    assert_eq!(requirements[0].probe_args, &["--version"]);
}

#[test]
fn cloud_alias_resolves_to_the_canonical_clo_row() {
    let alias = requirements_for_core("cloud");
    let canonical = requirements_for_core("clo");

    assert_eq!(alias, canonical);
    assert!(alias.iter().all(|requirement| requirement.core == "clo"));
    assert!(
        alias
            .iter()
            .any(|requirement| requirement.binary == "terraform")
    );
}

#[test]
fn app_core_requires_flutter_gradle_and_swift() {
    let binaries: Vec<&str> = requirements_for_core("app")
        .iter()
        .map(|requirement| requirement.binary)
        .collect();

    assert_eq!(binaries, vec!["flutter", "gradle", "swift"]);
}

#[test]
fn core_tools_outside_the_general_table_use_the_default_version_probe() {
    let terraform = requirements_for_core("clo")
        .into_iter()
        .find(|requirement| requirement.binary == "terraform")
        .unwrap();
    let flutter = requirements_for_core("app")
        .into_iter()
        .find(|requirement| requirement.binary == "flutter")
        .unwrap();

    assert_eq!(terraform.probe_args, &["--version"]);
    assert_eq!(flutter.probe_args, &["--version"]);
}

#[test]
fn hardware_core_requires_no_toolchain() {
    assert!(requirements_for_core("hardware").is_empty());
}

#[test]
fn unknown_core_requires_no_toolchain() {
    assert!(requirements_for_core("not-a-core").is_empty());
}
