//! Verify stable public root and module paths for Ecosystem.
//! Kiểm tra public path root và module ổn định của Ecosystem.

use mgc_types::Ecosystem;
use mgc_types::ecosystem::Ecosystem as ModuleEcosystem;

#[test]
fn crate_root_and_module_paths_export_the_same_type() {
    let module_value: ModuleEcosystem = Ecosystem::Cloud;

    assert_eq!(module_value, Ecosystem::Cloud);
}
