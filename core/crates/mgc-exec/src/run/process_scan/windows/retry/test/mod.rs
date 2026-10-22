//! Wire the nearest-owner Windows behavior tests.
//! Nối các test hành vi Windows tại owner gần nhất.

use super::super::*;
#[path = "windows_metadata_retry_observes_child_exit_after_clean_scan.rs"]
mod windows_metadata_retry_observes_child_exit_after_clean_scan;
#[path = "windows_metadata_retry_respects_command_deadline.rs"]
mod windows_metadata_retry_respects_command_deadline;
#[path = "windows_metadata_retry_stops_when_child_exits.rs"]
mod windows_metadata_retry_stops_when_child_exits;
