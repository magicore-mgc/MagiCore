//! Process-scan test wiring and qualified helpers.
//! Wiring test process-scan và helper có tên đầy đủ.

use super::{
    ProcessScanSnapshot, find_forbidden_descendant, find_forbidden_descendant_with_scope,
    forbidden_process_entry, live_descendant_identities,
};
mod process_scan;
use process_scan::process_group_matches;
