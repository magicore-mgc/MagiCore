//! Test-only fixtures used by the run integration owners.
//! Fixture test-only dùng chung cho các owner run integration.

#[path = "temporary_project.rs"]
mod temporary_project;
#[cfg(unix)]
#[path = "unix_process.rs"]
mod unix_process;

pub(super) use temporary_project::tmp_dir;
#[cfg(unix)]
pub(super) use unix_process::{
    fake_npm, force_kill_test_process_group, make_executable, require_python3,
    test_process_is_alive,
};
