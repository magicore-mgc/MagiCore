#![cfg(test)]
#![allow(clippy::unwrap_used)]

//! Select the adapter only for a detected CI/CD provider.
//! Chỉ chọn adapter khi nhận diện được provider CICD.

use crate::adapter::adapter_for;
use crate::provider::CicdProvider;
use tempfile::tempdir;

#[test]
fn factory_maps_a_wrangler_manifest_to_cloudflare() {
    let project = tempdir().unwrap();
    std::fs::write(
        project.path().join("wrangler.toml"),
        "name = 'worker'
",
    )
    .unwrap();
    assert_eq!(
        adapter_for(project.path()).map(|adapter| adapter.provider),
        Some(CicdProvider::Cloudflare)
    );
}
