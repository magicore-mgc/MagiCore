//! Cache runtime-policy owner wiring.
//! Wiring owner chính sách runtime cache.

mod runtime_policy;

pub(crate) use runtime_policy::bounded_positive_concurrency;
pub use runtime_policy::{
    current_unix_secs, download_concurrency_limit, extract_concurrency_limit,
    resolve_prefetch_enabled,
};

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
