//! Execution types facade — export request and report types.
//! Facade kiểu thực thi — export kiểu request và report.

#[path = "types.rs"]
mod types;
pub use types::{ExecOptions, ExecReport};
