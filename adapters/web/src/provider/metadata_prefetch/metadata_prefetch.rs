//! `metadata_prefetch.rs` — metadata prefetch.
//! Prefetch metadata đồng thời.

use super::super::provider::NpmDependencyProvider;
use crate::cache::metadata_concurrency_limit;
use crate::native;
use futures_util::stream::{self, StreamExt};
use mgc_resolver::DependencyError;
use mgc_types::PackageName;
use std::sync::Arc;

impl NpmDependencyProvider {
    pub async fn prefetch_resolution_metadata(
        &self,
        names: &[PackageName],
    ) -> Result<
        std::collections::HashMap<String, Arc<native::npm_registry::PackageMetadata>>,
        DependencyError,
    > {
        let mut results = std::collections::HashMap::new();
        let mut alias_to_source = Vec::new();
        let mut source_names = Vec::new();
        let mut seen_sources = std::collections::HashSet::new();
        let mut futures = Vec::new();

        for alias_name in names {
            let alias_name = alias_name.clone();
            let source_name = self.source_package_name(&alias_name);
            alias_to_source.push((alias_name.as_str().to_string(), source_name.clone()));
            if let Some(metadata) = self.metadata_cache.get(source_name.as_str()) {
                results.insert(alias_name.as_str().to_string(), metadata.clone());
                continue;
            }
            if seen_sources.insert(source_name.as_str().to_string()) {
                source_names.push(source_name);
            }
        }

        for source_name in source_names {
            futures.push(async move {
                let metadata = self.metadata(&source_name).await?;
                Ok::<_, DependencyError>((source_name.as_str().to_string(), metadata))
            });
        }

        let concurrency = metadata_concurrency_limit();
        let mut source_results = std::collections::HashMap::new();
        let mut metadata_errors = Vec::new();
        for fetched in stream::iter(futures)
            .buffer_unordered(concurrency)
            .collect::<Vec<_>>()
            .await
        {
            match fetched {
                Ok((source_name, metadata)) => {
                    source_results.insert(source_name, metadata);
                }
                Err(e) => metadata_errors.push(e),
            }
        }
        if let Some(e) = metadata_errors.into_iter().next() {
            return Err(e);
        }

        for (alias_name, source_name) in &alias_to_source {
            if results.contains_key(alias_name) {
                continue;
            }
            if let Some(metadata) = source_results.get(source_name.as_str()) {
                results.insert(alias_name.clone(), Arc::clone(metadata));
            }
        }

        Ok(results)
    }
}
