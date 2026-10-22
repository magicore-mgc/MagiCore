//! Tests for Web cache runtime policy.
//! Test chính sách runtime cache Web.

use tokio::sync::Semaphore;

use super::super::runtime_policy::{
    DEFAULT_DOWNLOAD_CONCURRENCY_LIMIT, MAX_WEB_CONCURRENCY_LIMIT, bounded_positive_concurrency,
    extract_concurrency_limit,
};

#[test]
fn accepted_override_stays_exact() {
    assert_eq!(bounded_positive_concurrency(Some("6"), 24), 6);
}

#[test]
fn invalid_zero_and_over_limit_values_use_fallback() {
    let fallback = 11;
    let too_large = Semaphore::MAX_PERMITS.saturating_add(1).to_string();
    assert_eq!(bounded_positive_concurrency(Some("0"), fallback), fallback);
    assert_eq!(
        bounded_positive_concurrency(Some("invalid"), fallback),
        fallback
    );
    assert_eq!(bounded_positive_concurrency(Some("65"), fallback), fallback);
    assert_eq!(
        bounded_positive_concurrency(Some(&too_large), fallback),
        fallback
    );
}

#[test]
fn download_default_and_extract_limit_are_valid() {
    assert_eq!(DEFAULT_DOWNLOAD_CONCURRENCY_LIMIT, 24);
    assert_eq!(MAX_WEB_CONCURRENCY_LIMIT, 64);
    let maximum = MAX_WEB_CONCURRENCY_LIMIT.min(Semaphore::MAX_PERMITS);
    let extract = extract_concurrency_limit();
    assert!((1..=maximum).contains(&extract));
    assert_eq!(bounded_positive_concurrency(Some("64"), 24), maximum);
}
