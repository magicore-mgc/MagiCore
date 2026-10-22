//! Nuget integrity test owner facade.
//! Facade owner test nuget integrity theo hành vi.

#[path = "nuget_integrity.rs"]
mod implementation;

pub(crate) use implementation::nuget_resolution_rejects_missing_malformed_or_ambiguous_sha512_markers;
