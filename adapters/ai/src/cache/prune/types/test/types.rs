//! Prune type cases — xác nhận các strategy và result public.

#![cfg(test)]

use super::types::{PruneResult, PruneStrategy};

#[test]
fn strategies_and_empty_result_keep_public_shape() {
    let strategies = [
        PruneStrategy::OlderThan(1),
        PruneStrategy::UnusedFor(2),
        PruneStrategy::KeepRecent(3),
        PruneStrategy::LargerThan(4),
    ];
    let result = PruneResult {
        removed_count: 0,
        bytes_freed: 0,
        removed_models: Vec::new(),
    };

    assert_eq!(strategies.len(), 4);
    assert_eq!(result.removed_count, 0);
}
