//! Public integration coverage for graph order, limits and conflicts.
//! Test integration public cho thứ tự, giới hạn và xung đột graph.

#![allow(clippy::unwrap_used)]
use async_trait::async_trait;
use mgc_resolver::protocols::{RegistryProtocol, ResolvedEntry};
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Duration;

struct ConflictingGraphProtocol;

#[async_trait]
impl RegistryProtocol for ConflictingGraphProtocol {
    async fn resolve(&self, name: &str, range: &str) -> mgc_types::MgResult<ResolvedEntry> {
        let (version, deps) = match name {
            "root" => (
                "1.0.0",
                vec![
                    ("left".to_string(), "*".to_string()),
                    ("right".to_string(), "*".to_string()),
                ],
            ),
            "left" => ("1.0.0", vec![("shared".to_string(), "^1".to_string())]),
            "right" => ("1.0.0", vec![("shared".to_string(), "^2".to_string())]),
            "shared" if range == "^1" => ("1.5.0", vec![]),
            "shared" if range == "^2" => ("2.1.0", vec![]),
            "shared" => ("2.1.0", vec![]),
            _ => {
                return Err(mgc_types::MgError::Other(format!(
                    "unexpected package {name}"
                )));
            }
        };
        Ok(ResolvedEntry {
            name: name.to_string(),
            version: version.to_string(),
            deps,
            artifact_url: format!("https://registry.invalid/{name}/{version}"),
            sha256: "00".repeat(32),
            extra_markers: vec![],
        })
    }

    async fn download(&self, _entry: &ResolvedEntry) -> mgc_types::MgResult<Vec<u8>> {
        Ok(vec![])
    }
}

#[tokio::test]
async fn graph_resolution_rejects_conflicting_transitive_ranges() {
    let error = ConflictingGraphProtocol
        .resolve_graph("root", "*")
        .await
        .expect_err("test operation must fail");
    assert!(matches!(error, mgc_types::MgError::DependencyConflict(_)));
    assert!(
        error
            .to_string()
            .contains("incompatible constraints for shared")
    );
}

struct ConcurrentGraphProtocol {
    active: Arc<AtomicUsize>,
    max_active: Arc<AtomicUsize>,
}

#[async_trait]
impl RegistryProtocol for ConcurrentGraphProtocol {
    async fn resolve(&self, name: &str, _range: &str) -> mgc_types::MgResult<ResolvedEntry> {
        let current = self.active.fetch_add(1, Ordering::SeqCst) + 1;
        self.max_active.fetch_max(current, Ordering::SeqCst);
        let delay_ms = name
            .strip_prefix("dep-")
            .and_then(|index| index.parse::<u64>().ok())
            .map(|index| 32 - index)
            .unwrap_or(1);
        tokio::time::sleep(Duration::from_millis(delay_ms)).await;
        self.active.fetch_sub(1, Ordering::SeqCst);

        let deps = if name == "root" {
            (0..32)
                .map(|index| (format!("dep-{index}"), "*".to_string()))
                .collect()
        } else {
            Vec::new()
        };
        Ok(ResolvedEntry {
            name: name.to_string(),
            version: "1.0.0".to_string(),
            deps,
            artifact_url: format!("https://registry.invalid/{name}/1.0.0"),
            sha256: "00".repeat(32),
            extra_markers: Vec::new(),
        })
    }

    async fn download(&self, _entry: &ResolvedEntry) -> mgc_types::MgResult<Vec<u8>> {
        Ok(Vec::new())
    }
}

#[tokio::test]
async fn graph_resolution_resolves_each_bfs_layer_concurrently_and_keeps_order() {
    let protocol = ConcurrentGraphProtocol {
        active: Arc::new(AtomicUsize::new(0)),
        max_active: Arc::new(AtomicUsize::new(0)),
    };

    let entries = protocol
        .resolve_graph("root", "*")
        .await
        .expect("test operation must succeed");

    assert_eq!(protocol.max_active.load(Ordering::SeqCst), 16);
    assert_eq!(entries.len(), 33);
    assert_eq!(entries[0].name, "root");
    for (index, entry) in entries.iter().skip(1).enumerate() {
        assert_eq!(entry.name, format!("dep-{index}"));
    }
}

#[tokio::test]
async fn graph_resolution_bounds_concurrent_roots_and_keeps_root_order() {
    let protocol = ConcurrentGraphProtocol {
        active: Arc::new(AtomicUsize::new(0)),
        max_active: Arc::new(AtomicUsize::new(0)),
    };
    let roots = (0..32)
        .map(|index| (format!("dep-{index}"), "*".to_string()))
        .collect::<Vec<_>>();

    let entries = protocol
        .resolve_graph_roots(&roots)
        .await
        .expect("test operation must succeed");

    assert!(protocol.max_active.load(Ordering::SeqCst) > 1);
    assert!(protocol.max_active.load(Ordering::SeqCst) <= 16);
    assert_eq!(entries.len(), roots.len());
    for (entry, (root, _)) in entries.iter().zip(roots) {
        assert_eq!(entry.name, root);
    }
}
