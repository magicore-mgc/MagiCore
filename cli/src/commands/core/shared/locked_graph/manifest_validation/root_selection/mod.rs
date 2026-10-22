//! Manifest-root selection owner wiring.
//! Wiring owner chọn root theo manifest.

#[path = "root_selection.rs"]
mod implementation;

pub(crate) use implementation::{ManifestRootSelection, SelectedRoots, select_manifest_roots};
