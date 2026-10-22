//! Merge trust roots and resolve the fail-closed unsigned policy.
//! Gộp trust root và phân giải chính sách unsigned theo fail-closed.

mod provenance;
#[cfg(test)]
pub(in crate::commands) use provenance::merge_roots;
pub use provenance::{ProvenanceMode, resolve_provenance_mode, trust_roots};
