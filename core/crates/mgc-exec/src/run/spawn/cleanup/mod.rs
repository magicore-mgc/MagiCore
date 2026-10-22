//! Cleanup-result aggregation facade for the shared spawn pipeline.
//! Facade gom kết quả cleanup cho luồng spawn dùng chung.

#[path = "cleanup.rs"]
mod implementation;

pub(super) use implementation::{
    combine_execution_and_environment_cleanup, combine_execution_and_handle_cleanup,
};
