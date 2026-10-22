//! Pair the standard solve output with exact direct-root selections.
//! Ghép output solve chuẩn với lựa chọn root trực tiếp chính xác.

use super::super::{RootResolution, SolveResult};

/// Resolver output with root identity metadata for policy-aware callers.
/// Kết quả resolver kèm metadata root cho caller áp dụng policy.
#[derive(Debug, Clone)]
pub struct SolveResultWithRoots {
    pub result: SolveResult,
    pub root_resolutions: Vec<RootResolution>,
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
