//! Wire the nearest-owner Windows behavior tests.
//! Nối các test hành vi Windows tại owner gần nhất.

use super::super::*;
#[path = "windows_process_scan_fails_closed_on_child_below_reused_pid.rs"]
mod windows_process_scan_fails_closed_on_child_below_reused_pid;
#[path = "windows_process_scan_ignores_unrelated_process_after_pid_reuse.rs"]
mod windows_process_scan_ignores_unrelated_process_after_pid_reuse;
#[path = "windows_process_scan_keeps_ancestry_across_clean_snapshots.rs"]
mod windows_process_scan_keeps_ancestry_across_clean_snapshots;
