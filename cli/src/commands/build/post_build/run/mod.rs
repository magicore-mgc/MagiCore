//! Post-build runner owner.
//! Owner chạy bước sau build.

#[path = "run.rs"]
mod implementation;

pub(crate) use implementation::{PostBuildOptions, project_version, run, run_for_core};
