//! Python owner-selection and site-path tests.
//! Test chọn owner và đường dẫn site Python.

use super::support::*;

#[test]
fn python_owner_selection_isolated_for_multi_core_lock_and_rejects_ambiguous_legacy_entries() {
    let mut ai = package("shared-name", "1.0.0", EcosystemTag::Python);
    ai.owner_core = Some("ai".to_string());
    let mut lib = package("shared-name", "2.0.0", EcosystemTag::Python);
    lib.owner_core = Some("lib".to_string());
    let lock = vec![ai, lib];

    let ai_selected = select_python_packages_for_owner(&lock, "ai").unwrap();
    let lib_selected = select_python_packages_for_owner(&lock, "lib").unwrap();
    assert_eq!(ai_selected.len(), 1);
    assert_eq!(ai_selected[0].version, "1.0.0");
    assert_eq!(lib_selected.len(), 1);
    assert_eq!(lib_selected[0].version, "2.0.0");

    let ambiguous = vec![
        lock[0].clone(),
        package("legacy", "1.0.0", EcosystemTag::Python),
    ];
    assert!(
        select_python_packages_for_owner(&ambiguous, "ai")
            .unwrap_err()
            .to_string()
            .contains("ambiguous core ownership")
    );
}

#[test]
fn python_site_path_rejects_untrusted_lock_traversal_segments() {
    assert!(python_site_dirname("six", "1.17.0").is_ok());
    for (name, version) in [
        ("../../outside", "1.0.0"),
        ("pkg", "1.0.0-../../outside"),
        ("pkg", r"1.0.0-..\outside"),
    ] {
        assert!(
            python_site_dirname(name, version).is_err(),
            "must reject {name}@{version}"
        );
    }
}
