//! Lock completion lifecycle tests shared across install owners.
//! Test lifecycle hoàn thành lock dùng chung giữa các install owner.

use super::support::*;

#[test]
fn delta_install_completes_its_lock_from_existing_exact_pins() {
    let graph = ResolvedGraph {
        packages: vec![
            resolved("existing-dep", "1.0.0"),
            resolved("new-dep", "2.0.0"),
        ],
    };
    let existing = Lockfile {
        packages: vec![package("existing-dep", "1.0.0", EcosystemTag::Python)],
        ..Lockfile::new()
    };
    let mut delta = vec![package("new-dep", "2.0.0", EcosystemTag::Python)];

    complete_lock_packages_from_existing(&graph, EcosystemTag::Python, &mut delta, &existing);

    validate_lock_coverage(&graph, EcosystemTag::Python, &delta).unwrap();
    assert_eq!(delta.len(), 2);
}
