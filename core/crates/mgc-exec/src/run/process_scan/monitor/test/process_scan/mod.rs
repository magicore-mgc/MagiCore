//! Process-scan test wiring and qualified helpers.
//! Wiring test process-scan và helper có tên đầy đủ.

use super::{
    ProcessScanSnapshot, find_forbidden_descendant, forbidden_process_entry,
    live_descendant_identities,
};
// Used only by the macOS-gated scan tests; allow the Linux build to keep
// the import without an unused warning.
// (Chỉ test scan gate macOS dùng; allow để build Linux không cảnh báo unused.)
#[allow(unused_imports)]
use super::find_forbidden_descendant_with_scope;
mod process_scan;
use process_scan::process_group_matches;
