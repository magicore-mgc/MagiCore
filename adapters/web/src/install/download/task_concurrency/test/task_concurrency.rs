//! Tests for Web pipeline concurrency bounds.
//! Test giới hạn concurrency pipeline Web.

use tokio::sync::Semaphore;

use super::configured_pipeline_task_limit;

#[test]
fn preserves_explicit_valid_pipeline_limit() {
    assert_eq!(configured_pipeline_task_limit(Some("7"), 24, 8), 7);
}

#[test]
fn invalid_or_zero_value_uses_safe_fallback() {
    assert_eq!(configured_pipeline_task_limit(Some("0"), 24, 8), 32);
    assert_eq!(configured_pipeline_task_limit(Some("bad"), 24, 8), 32);
}

#[test]
fn over_platform_limit_falls_back_instead_of_panicking() {
    let too_large = Semaphore::MAX_PERMITS.saturating_add(1).to_string();
    let limit = configured_pipeline_task_limit(Some(&too_large), 24, 8);
    assert_eq!(limit, 32);
    assert!(limit <= Semaphore::MAX_PERMITS);
}

#[test]
fn fallback_sum_saturates_and_stays_within_semaphore_limit() {
    let limit = configured_pipeline_task_limit(None, usize::MAX, usize::MAX);
    assert_eq!(limit, 64.min(Semaphore::MAX_PERMITS));
}
