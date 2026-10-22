//! `prefetch_versions.rs` — prefetch versions.
//! Prefetch danh sách version.

use super::super::provider::NpmDependencyProvider;
use mgc_resolver::DependencyError;
use mgc_types::{PackageName, Version};

impl NpmDependencyProvider {
    pub(in crate::provider) async fn prefetch_versions_impl(
        &self,
        packages: &[PackageName],
    ) -> Result<Vec<(PackageName, Vec<Version>)>, DependencyError> {
        let mut results = Vec::with_capacity(packages.len());
        let mut missing = Vec::new();

        for package in packages {
            if let Some(cached) = self.cached_versions_for(package) {
                results.push((package.clone(), cached));
                continue;
            }

            let package_key = self.source_package_name(package).as_str().to_string();
            if let Some(metadata) = self.metadata_cache.get(&package_key) {
                let versions = self.versions_with_age_gate(package, &metadata)?;
                self.insert_versions_for(package, versions.clone());
                results.push((package.clone(), versions));
                continue;
            }
            missing.push(package.clone());
        }

        if missing.is_empty() {
            return Ok(results);
        }

        let fetched_metadata = self.prefetch_resolution_metadata(&missing).await?;
        for package in missing {
            let Some(metadata) = fetched_metadata.get(package.as_str()) else {
                return Err(DependencyError(format!(
                    "prefetch metadata missing result for '{}'",
                    package.as_str()
                )));
            };
            let versions = self.versions_with_age_gate(&package, metadata)?;
            self.insert_versions_for(&package, versions.clone());
            results.push((package.clone(), versions));
        }

        Ok(results)
    }
}
