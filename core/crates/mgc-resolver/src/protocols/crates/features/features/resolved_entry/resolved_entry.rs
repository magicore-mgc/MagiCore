//! Convert resolved crate graph state into the public registry payload.
//! Chuyển trạng thái graph crate thành payload registry public.

use super::super::super::super::{CrateResolutionState, CratesProtocol, ResolvedEntry};
use super::{ResolvedEntryError, ResolvedEntryResult};

pub(in crate::protocols::crates::features::features) fn resolved_entry_for_state(
    protocol: &CratesProtocol,
    state: &CrateResolutionState,
) -> ResolvedEntryResult<ResolvedEntry> {
    let sha256 = state
        .entry
        .cksum
        .as_deref()
        .ok_or(ResolvedEntryError::MissingChecksum)?
        .to_ascii_lowercase();
    Ok(ResolvedEntry {
        name: state.entry.name.clone(),
        version: state.entry.vers.clone(),
        deps: state
            .resolved_dependencies
            .iter()
            .map(|(name, version)| (name.clone(), format!("={version}")))
            .collect(),
        artifact_url: super::super::super::super::index::download_url_for(
            protocol,
            &state.entry.name,
            &state.entry.vers,
        ),
        sha256,
        extra_markers: state.extra_markers.iter().cloned().collect(),
    })
}
