//! Retry policy.
use anyhow::Result;
// Retry giới hạn và backoff hiện có.

use crate::native::npm_registry::network_profile_log;
use std::time::Duration;

pub(in crate::native::npm_registry) fn jitter_ms(attempt: u32) -> u64 {
    let state = (attempt as u64)
        .wrapping_mul(6364136223846793005)
        .wrapping_add(1442695040888963407);
    (state >> 33) % 100
}

pub(in crate::native::npm_registry) async fn with_retry<F, Fut, T>(
    kind: &str,
    target: &str,
    mut f: F,
) -> Result<T>
where
    F: FnMut() -> Fut,
    Fut: std::future::Future<Output = Result<T>>,
{
    let mut last_error = None;
    for attempt in 0..4u32 {
        match f().await {
            Ok(value) => {
                if attempt > 0 {
                    network_profile_log(
                        kind,
                        target,
                        &format!("success_after_retry attempt={}", attempt + 1),
                    );
                }
                return Ok(value);
            }
            Err(err) => {
                network_profile_log(
                    kind,
                    target,
                    &format!("retry attempt={} error={}", attempt + 1, err),
                );
                last_error = Some(err);
                if attempt < 3 {
                    let base_ms = 50u64 * (2u64.pow(attempt));
                    let jitter = jitter_ms(attempt);
                    tokio::time::sleep(Duration::from_millis(base_ms + jitter)).await;
                }
            }
        }
    }
    // The loop above always runs at least once and returns on the first
    // success, so a fall-through guarantees an error was captured. The
    // `None` arm is unreachable by construction — it carries a real error
    // anyway so a future refactor can never turn it into a panic.
    match last_error {
        Some(error) => Err(error),
        None => Err(anyhow::anyhow!(
            "retry loop for {kind} {target} exhausted attempts without capturing an error"
        )),
    }
}
