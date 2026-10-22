//! Candidate discovery exports — xuất owner tìm entry prune trực tiếp trong cache.

mod candidate_discovery;

pub(super) use candidate_discovery::{CacheCandidate, discover_candidates};

#[cfg(test)]
#[path = "test/candidate_discovery.rs"]
mod tests;
