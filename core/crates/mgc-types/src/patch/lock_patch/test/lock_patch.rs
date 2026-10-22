//! Verify the lockfile patch-record serialization contract.
//! Kiểm tra contract serialization của record patch lockfile.

#![cfg(test)]
#![allow(clippy::unwrap_used)]

use super::LockPatch;

#[test]
fn lock_patch_serializes() {
    let lp = LockPatch::new(
        "react".into(),
        "1.0.0".into(),
        "sha256-def".into(),
        "2026-01-01T00:00:00Z".into(),
    );
    let json = serde_json::to_string(&lp).unwrap();
    let back: LockPatch = serde_json::from_str(&json).unwrap();
    assert_eq!(back.name, "react");
    assert_eq!(back.sha256, "sha256-def");
}
