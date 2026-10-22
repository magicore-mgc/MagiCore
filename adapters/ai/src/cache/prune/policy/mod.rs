//! Prune policy exports — xuất logic chọn entry theo strategy.

mod policy;

pub(super) use policy::select_removals;

#[cfg(test)]
#[path = "test/policy.rs"]
mod tests;
