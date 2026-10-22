//! Read Web cache-related runtime environment policy and wall-clock time.
//! Đọc cấu hình runtime liên quan cache Web và thời gian hệ thống.

pub(in crate::cache::runtime_policy) const DEFAULT_DOWNLOAD_CONCURRENCY_LIMIT: usize = 24;
const DEFAULT_EXTRACT_CONCURRENCY_CAP: usize = 32;
pub(in crate::cache::runtime_policy) const MAX_WEB_CONCURRENCY_LIMIT: usize = 64;

pub(crate) fn bounded_positive_concurrency(raw: Option<&str>, fallback: usize) -> usize {
    let maximum = MAX_WEB_CONCURRENCY_LIMIT.min(tokio::sync::Semaphore::MAX_PERMITS);
    let fallback = fallback.clamp(1, maximum);
    raw.and_then(|value| value.trim().parse::<usize>().ok())
        .filter(|limit| *limit > 0 && *limit <= maximum)
        .unwrap_or(fallback)
}

pub fn download_concurrency_limit() -> usize {
    let configured = std::env::var("MAGICORE_WEB_DOWNLOAD_CONCURRENCY").ok();
    bounded_positive_concurrency(configured.as_deref(), DEFAULT_DOWNLOAD_CONCURRENCY_LIMIT)
}

pub fn extract_concurrency_limit() -> usize {
    let configured = std::env::var("MAGICORE_WEB_EXTRACT_CONCURRENCY").ok();
    let fallback = std::thread::available_parallelism()
        .map(|parallelism| parallelism.get())
        .unwrap_or(4)
        .min(DEFAULT_EXTRACT_CONCURRENCY_CAP);
    bounded_positive_concurrency(configured.as_deref(), fallback)
}

pub fn resolve_prefetch_enabled() -> bool {
    std::env::var("MAGICORE_WEB_RESOLVE_PREFETCH")
        .ok()
        .map(|value| value.trim().to_ascii_lowercase())
        .is_some_and(|value| matches!(value.as_str(), "1" | "true" | "yes" | "on"))
}

pub fn current_unix_secs() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}
