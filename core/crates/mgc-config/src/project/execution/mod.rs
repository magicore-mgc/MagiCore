//! Project execution profile configuration.
//! Cấu hình profile thực thi của dự án.

#[path = "execution.rs"]
mod execution;

pub use execution::ProjectExecutionConfig;
pub(super) use execution::default_execution_for;

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
