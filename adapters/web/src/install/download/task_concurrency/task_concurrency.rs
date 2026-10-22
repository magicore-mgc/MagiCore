//! Resolve a bounded pipeline task concurrency limit.
//! Tính giới hạn concurrency task pipeline an toàn.

use crate::cache::{bounded_positive_concurrency, download_concurrency_limit};

fn configured_pipeline_task_limit(
    configured: Option<&str>,
    download_limit: usize,
    extract_limit: usize,
) -> usize {
    bounded_positive_concurrency(configured, download_limit.saturating_add(extract_limit))
}

pub fn pipeline_task_concurrency_limit(extract_concurrency: usize) -> usize {
    let configured = std::env::var("MAGICORE_WEB_PIPELINE_TASK_CONCURRENCY").ok();
    configured_pipeline_task_limit(
        configured.as_deref(),
        download_concurrency_limit(),
        extract_concurrency,
    )
}

#[cfg(test)]
#[path = "test/task_concurrency.rs"]
mod tests;
