//! Embedded registry contract tests.
//! Kiểm alias/kernel thật, không chấp nhận registry rỗng như một lần pass.
use super::*;

#[test]
fn vanilla_alias_and_canonical_layer_share_the_current_archive() {
    let kernels = EmbeddedKernel::kernel_map();
    let alias = kernels.get("web/vanilla").unwrap();
    let canonical = kernels.get("web/frontend/vanilla").unwrap();
    assert_eq!(alias.data, canonical.data);
    assert_eq!(alias.version, canonical.version);
}

#[test]
fn available_archive_registry_is_nonempty_and_has_layer_paths() {
    let available = EmbeddedKernel::list_available();
    assert!(!available.is_empty());
    for kernel in available {
        assert!(kernel.contains('/'), "Kernel should be core/name format");
    }
}
