//! Generic native outdated query wiring.
//! Nối query outdated native dùng chung.

mod candidate_manifest;
mod graph_compare;
mod native;
pub(super) use native::query_outdated_native;
