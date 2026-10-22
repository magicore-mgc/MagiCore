//! Select entries for a prune strategy — chọn entry cần xóa theo policy.

use crate::cache::prune::candidate_discovery::CacheCandidate;
use crate::cache::prune::types::PruneStrategy;

pub(in crate::cache::prune) fn select_removals(
    candidates: &[CacheCandidate],
    strategy: PruneStrategy,
) -> Vec<usize> {
    match strategy {
        PruneStrategy::OlderThan(days) => candidates
            .iter()
            .enumerate()
            .filter_map(|(index, candidate)| {
                candidate
                    .metadata
                    .as_ref()
                    .is_some_and(|metadata| metadata.age_days() > days)
                    .then_some(index)
            })
            .collect(),
        PruneStrategy::UnusedFor(days) => candidates
            .iter()
            .enumerate()
            .filter_map(|(index, candidate)| {
                candidate
                    .metadata
                    .as_ref()
                    .is_some_and(|metadata| metadata.days_since_access() > days)
                    .then_some(index)
            })
            .collect(),
        PruneStrategy::LargerThan(bytes) => candidates
            .iter()
            .enumerate()
            .filter_map(|(index, candidate)| {
                candidate
                    .content_bytes
                    .is_some_and(|content_bytes| content_bytes > bytes)
                    .then_some(index)
            })
            .collect(),
        PruneStrategy::KeepRecent(keep_count) => keep_recent_removals(candidates, keep_count),
    }
}

fn keep_recent_removals(candidates: &[CacheCandidate], keep_count: usize) -> Vec<usize> {
    let mut known = candidates
        .iter()
        .enumerate()
        .filter_map(|(index, candidate)| {
            candidate
                .metadata
                .as_ref()
                .map(|metadata| (index, metadata.last_accessed, &candidate.path))
        })
        .collect::<Vec<_>>();

    known.sort_by(|left, right| right.1.cmp(&left.1).then_with(|| left.2.cmp(right.2)));

    known
        .into_iter()
        .skip(keep_count)
        .map(|(index, _, _)| index)
        .collect()
}
