//! Verify the stable public paths for shared patch contracts.
//! Kiểm tra các public path ổn định của contract patch dùng chung.

use mgc_types::package::VersionRange;
use mgc_types::patch::{
    LockPatch as ModuleLockPatch, PatchKind as ModulePatchKind, PatchSpec as ModulePatchSpec,
};
use mgc_types::{LockPatch, PatchKind, PatchSpec};

#[test]
fn crate_root_and_module_paths_export_the_same_patch_types() {
    let module_spec: ModulePatchSpec = PatchSpec::new(
        "react".into(),
        VersionRange::star(),
        "patches/react.patch".into(),
        "sha256-abc".into(),
    );
    let module_kind: ModulePatchKind = PatchKind::Diff;
    let module_lock: ModuleLockPatch = LockPatch::new(
        "react".into(),
        "1.0.0".into(),
        "sha256-def".into(),
        "2026-01-01T00:00:00Z".into(),
    );

    assert_eq!(module_spec.kind, module_kind);
    assert_eq!(module_lock.name, "react");
}
