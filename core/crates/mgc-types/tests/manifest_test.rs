//! Verify the stable public Manifest exports.
//! Kiểm tra các export public ổn định của Manifest.

use mgc_types::Manifest as RootManifest;
use mgc_types::ecosystem::Ecosystem;
use mgc_types::manifest::Manifest as ModuleManifest;

#[test]
fn crate_root_and_module_paths_export_the_same_manifest() {
    let root = RootManifest::new("public-api", Ecosystem::Web);
    let module: ModuleManifest = root.clone();

    assert_eq!(root, module);
}
