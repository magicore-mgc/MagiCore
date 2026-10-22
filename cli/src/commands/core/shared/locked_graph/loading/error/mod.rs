//! Local error owner for locked-graph loading.
//! Owner lỗi cục bộ cho luồng nạp lock graph.

#[path = "error.rs"]
mod implementation;

pub(crate) use implementation::LockedGraphLoadingError;

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
