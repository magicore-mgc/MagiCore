//! Integration facade for resolver patch API coverage.
//! Facade integration cho API patch của resolver.

#[path = "patches/apply_patch.rs"]
mod apply_patch;
#[path = "patches/path_safety.rs"]
mod path_safety;
#[path = "patches/staged_validation.rs"]
mod staged_validation;

#[test]
fn public_patch_routes_are_reachable() {
    use std::path::Path;
    let root = Path::new("project-root");
    let resolved = mgc_resolver::get_patches_dir(Some(root)).expect("project path resolves");
    let expected = mgc_platform::paths::ProjectPaths::from_root(root)
        .patches_dir()
        .to_path_buf();
    assert_eq!(resolved, expected);
    let _apply: fn(&Path, &Path) -> anyhow::Result<Vec<std::path::PathBuf>> =
        mgc_resolver::apply_patch;
    let _verify: fn(&Path, &str) -> anyhow::Result<bool> = mgc_resolver::verify_patch_integrity;
}
