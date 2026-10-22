#![cfg(test)]
#![allow(clippy::unwrap_used)]

//! An empty JavaScript manifest resolves without contacting a package registry.
//! Manifest JavaScript rỗng resolve mà không gọi package registry.

use crate::native::rn_layers::resolve_javascript::resolve_javascript;
use mgc_types::{Ecosystem, Manifest};
use tempfile::TempDir;

#[tokio::test]
async fn empty_manifest_returns_an_empty_graph() {
    let temp = TempDir::new().unwrap();
    let manifest = Manifest::new("rn-empty", Ecosystem::App);
    let graph = resolve_javascript(&manifest, temp.path()).await.unwrap();
    assert!(graph.packages.is_empty());
}
