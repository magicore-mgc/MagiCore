//! Tests for CI/CD provider marker handling.
//! Test cách xử lý marker provider CI/CD.

use super::super::provider_from_markers;
use crate::provider::CicdProvider;
use std::fs;

#[test]
fn first_marker_in_the_existing_order_wins() {
    let project = tempfile::tempdir().expect("temporary directory");
    fs::write(project.path().join("wrangler.toml"), "name = \"worker\"\n")
        .expect("write Cloudflare marker");
    fs::create_dir_all(project.path().join("argocd")).expect("create ArgoCD marker directory");
    fs::write(
        project.path().join("argocd").join("application.yaml"),
        "kind: Application\n",
    )
    .expect("write ArgoCD marker");

    assert_eq!(
        provider_from_markers(project.path()),
        Some(CicdProvider::Cloudflare)
    );
}

#[test]
fn no_existing_marker_returns_none() {
    let project = tempfile::tempdir().expect("temporary directory");
    assert_eq!(provider_from_markers(project.path()), None);
}
