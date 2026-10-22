//! `types.rs` — Private resolution-stage types / type nội bộ của các bước resolve.
use crate::native::npm_registry::PackageMetadata;
use mgc_resolver::solver::{Resolution, RootResolution};
use std::collections::HashMap;
use std::sync::Arc;

pub(in crate::adapter::resolution) type ResolutionMetadata = HashMap<String, Arc<PackageMetadata>>;
pub(in crate::adapter::resolution) type ResolutionIndex<'a> = HashMap<String, Vec<&'a Resolution>>;

pub(in crate::adapter::resolution) struct ResolutionSnapshot {
    pub(in crate::adapter::resolution) resolutions: Vec<Resolution>,
    pub(in crate::adapter::resolution) root_resolutions: Vec<RootResolution>,
    pub(in crate::adapter::resolution) metadata: ResolutionMetadata,
}
