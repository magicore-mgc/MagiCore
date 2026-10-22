//! Configure the bounded Rayon pool used outside nested Rayon callers.
//! Cấu hình Rayon pool có giới hạn cho caller không chạy trong Rayon.

use std::fmt;
use std::sync::{Arc, OnceLock};

use mgc_types::{MgError, MgResult};

use super::super::error::{LinkTreeError, LinkTreeOperation};

#[derive(Debug)]
struct PoolBuildSource(Arc<rayon::ThreadPoolBuildError>);

impl fmt::Display for PoolBuildSource {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self.0.as_ref(), formatter)
    }
}

impl std::error::Error for PoolBuildSource {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(self.0.as_ref())
    }
}

pub fn hardlink_thread_count() -> usize {
    std::env::var("MAGICORE_WEB_HARDLINK_THREADS")
        .ok()
        .and_then(|value| value.trim().parse::<usize>().ok())
        .filter(|&count| count > 0)
        .unwrap_or(default_hardlink_threads())
}

pub fn default_hardlink_threads() -> usize {
    std::thread::available_parallelism()
        .map(|n| n.get().clamp(1, 6))
        .unwrap_or(2)
}

pub fn hardlink_pool() -> MgResult<&'static rayon::ThreadPool> {
    static POOL: OnceLock<Result<rayon::ThreadPool, Arc<rayon::ThreadPoolBuildError>>> =
        OnceLock::new();
    let pool = POOL.get_or_init(|| {
        rayon::ThreadPoolBuilder::new()
            .num_threads(hardlink_thread_count())
            .thread_name(|index| format!("mgc-web-hardlink-{index}"))
            .build()
            .map_err(Arc::new)
    });
    pool.as_ref().map_err(|err| {
        MgError::from(LinkTreeError::with_source(
            LinkTreeOperation::InitializePool,
            format!("failed to initialize hardlink thread pool: {err}"),
            PoolBuildSource(Arc::clone(err)),
        ))
    })
}
