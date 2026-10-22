//! Verify the public configuration-time patch spec contract.
//! Kiểm tra contract spec patch public ở thời điểm cấu hình.

#![cfg(test)]
#![allow(clippy::unwrap_used)]

use super::{PatchKind, PatchSpec};
use crate::package::VersionRange;

#[test]
fn patch_spec_serializes() {
    let vr = VersionRange::parse("^1.0.0").unwrap();
    let spec = PatchSpec::new(
        "react".into(),
        vr,
        "patches/react.patch".into(),
        "sha256-abc".into(),
    );
    let json = serde_json::to_string(&spec).unwrap();
    assert!(json.contains("react"));
    assert!(json.contains("sha256-abc"));
    let back: PatchSpec = serde_json::from_str(&json).unwrap();
    assert_eq!(back.kind, PatchKind::Diff);
}
