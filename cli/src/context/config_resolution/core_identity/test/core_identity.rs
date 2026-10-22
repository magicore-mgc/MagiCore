//! Verify canonical core aliases preserve the same project identity.
//! Kiểm tra alias core được chuẩn hóa vẫn giữ nguyên identity project.

use super::*;

#[test]
fn cloud_alias_confirms_the_canonical_cloud_identity() {
    ensure_override_matches_identity("clo", "cloud")
        .expect("cloud alias must confirm the persisted cloud identity");
}

#[test]
fn mismatching_core_returns_the_local_typed_conflict() {
    let error = ensure_override_matches_identity("ai", "web")
        .expect_err("different cores must not share persisted project identity");

    assert_eq!(
        error.to_string(),
        "core override 'ai' conflicts with existing project core 'web'; core reassignment is not supported by this version"
    );
    assert!(
        error
            .downcast_ref::<super::super::super::CoreOverrideConflictError>()
            .is_some()
    );
}
