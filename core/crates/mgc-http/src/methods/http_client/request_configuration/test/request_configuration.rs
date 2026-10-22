//! Request configuration state tests.
//! Test state cấu hình request.

use super::*;

#[test]
fn client_builders_store_their_existing_policy_values() {
    let cache = HttpCache::new();
    let client = HttpClient::new()
        .expect("build local HTTP client")
        .with_auth("authorization", "Bearer test-token")
        .with_retry(RetryStrategy::Fixed(Duration::from_millis(7)))
        .with_ratelimit(4, Duration::from_secs(2))
        .with_cache(cache);

    let auth = client
        .auth
        .as_ref()
        .map(|(name, value)| (name.as_str(), value.as_str()));
    assert_eq!(auth, Some(("authorization", "Bearer test-token")));
    assert_eq!(client.retry.delay(0), Duration::from_millis(7));
    assert!(client.ratelimit.is_some());
    assert!(client.cache.is_some());
}
