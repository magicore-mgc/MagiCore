#![cfg(test)]
#![allow(clippy::unwrap_used)]

//! Verify missing iOS lock input fails before any registry network work.
//! Xác nhận thiếu lock iOS sẽ lỗi trước khi gọi registry qua mạng.

use crate::native::rn_layers::resolve_rn_layers;
use mgc_types::{Ecosystem, Manifest, MgError};
use tempfile::TempDir;

#[tokio::test]
async fn podfile_without_lock_fails_before_tier_resolution() {
    let temp = TempDir::new().unwrap();
    std::fs::write(temp.path().join("Podfile"), "platform :ios, '13.0'\n").unwrap();
    let manifest = Manifest::new("rn-app", Ecosystem::App);

    let error = resolve_rn_layers(&manifest, temp.path())
        .await
        .err()
        .expect("missing iOS lock must fail before resolving tiers");
    assert!(matches!(error, MgError::Unsupported { .. }));
    assert!(error.to_string().contains("Podfile.lock"));
}
