//! Process-scan test wiring and qualified helpers.
//! Wiring test process-scan và helper có tên đầy đủ.

use super::{
    ProcessScanSnapshot, find_forbidden_descendant, forbidden_process_entry,
    live_descendant_identities,
};
// Only the macOS-gated scan tests call the scope-aware variant.
// (Chỉ các test scan gate macOS gọi biến thể có scope.)
#[cfg(target_os = "macos")]
use super::find_forbidden_descendant_with_scope;
mod process_scan;
use process_scan::process_group_matches;
