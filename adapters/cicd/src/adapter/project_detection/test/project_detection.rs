#![cfg(test)]
#![allow(clippy::unwrap_used)]

//! Recognize provider markers and reject empty directories.
//! Nhận diện marker provider và từ chối thư mục rỗng.

use crate::adapter::CicdAdapter;
use crate::provider::CicdProvider;
use mgc_types::capabilities::ProjectDetector;
use tempfile::tempdir;

#[test]
fn detector_uses_provider_manifest_identity() {
    let project = tempdir().unwrap();
    let adapter = CicdAdapter {
        provider: CicdProvider::Cloudflare,
    };
    assert!(!adapter.can_handle(project.path()));
    std::fs::write(
        project.path().join("wrangler.toml"),
        "name = 'worker'
",
    )
    .unwrap();
    assert!(adapter.can_handle(project.path()));
}
