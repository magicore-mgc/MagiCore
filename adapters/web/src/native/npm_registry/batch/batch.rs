//! Batch operations.
// Các thao tác registry theo lô.

use crate::native::npm_registry::{NpmRegistry, PackageMetadata};
use anyhow::Result;

/// Parallel metadata fetcher — fires all requests concurrently and collects results.
/// Returns errors individually so a single bad package does not abort the batch.
pub async fn batch_fetch_metadata(
    registry_url: &str,
    packages: &[&str],
) -> Vec<(String, Result<PackageMetadata>)> {
    use futures_util::future::join_all;

    let futures: Vec<_> = packages
        .iter()
        .map(|name| {
            let reg = NpmRegistry::new(registry_url);
            let name = name.to_string();
            async move {
                let result = reg.fetch_metadata(&name).await;
                (name, result)
            }
        })
        .collect();

    join_all(futures).await
}
