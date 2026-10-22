//! RegistryProtocol entry points and ordered PyPI resolution coordinator.
//! Entry point RegistryProtocol và coordinator resolve PyPI có thứ tự ổn định.

use crate::protocols::pypi::{PypiProtocol, pep440_matches, select_file};
use crate::protocols::{RegistryProtocol, ResolvedEntry};
use async_trait::async_trait;
use mgc_types::{MgError, MgResult, Version};

use super::artifact_download::download as download_artifact;
use super::artifact_markers::project_artifact_markers;
use super::dependency_projection::project_dependencies;
use super::python_release_selection::select_compatible_release;
use super::release_candidates::load_candidates;
use super::selected_metadata::load_selected_metadata;

#[async_trait]
impl RegistryProtocol for PypiProtocol {
    fn matches_requirement(&self, range: &str, version: &Version) -> bool {
        pep440_matches(range, version)
    }

    async fn resolve(&self, name: &str, range: &str) -> MgResult<ResolvedEntry> {
        resolve_entry(self, name, range).await
    }

    async fn download(&self, entry: &ResolvedEntry) -> MgResult<Vec<u8>> {
        download_artifact(&self.client, entry).await
    }
}

async fn resolve_entry(
    protocol: &PypiProtocol,
    name: &str,
    range: &str,
) -> MgResult<ResolvedEntry> {
    let candidates = load_candidates(protocol, name, range).await?;
    let selected = select_compatible_release(protocol, name, range, candidates).await?;
    let file = select_file(&selected.files, selected.consumer).ok_or_else(|| {
        MgError::Other(format!(
            "no downloadable file for {name} {}",
            selected.version
        ))
    })?;

    let mut markers = project_artifact_markers(file);
    let version_info =
        load_selected_metadata(protocol, name, &selected.version, selected.version_info).await?;
    let deps = project_dependencies(
        version_info.requires_dist,
        name,
        &selected.version,
        &mut markers,
    )?;

    Ok(ResolvedEntry {
        name: name.to_string(),
        version: selected.version.to_string(),
        deps,
        artifact_url: file.url.clone(),
        sha256: file.digests.sha256.clone(),
        extra_markers: markers,
    })
}
