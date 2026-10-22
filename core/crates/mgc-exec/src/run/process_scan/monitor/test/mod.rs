//! Unix monitor test wiring and shared test API.
//! Wiring test monitor Unix và API test dùng chung.

use super::super::super::super::termination::terminate_process_tree;
use super::super::super::super::{
    ForbiddenProcess, NPM_VERSION_PROBE_SHIM_CONTENT, ShadowPath, is_npm_version_shell_probe,
};
use super::super::super::ProcessScanSnapshot;
use super::{
    find_forbidden_descendant, find_forbidden_descendant_with_scope, forbidden_process_entry,
    is_allowed_shadow_npm_version_probe, is_generated_shadow_npm_version_probe,
    is_shadow_npm_version_probe, live_descendant_identities,
    npm_probe_command_line_matches_shim_path,
};
mod helpers;
use helpers::{
    find_forbidden_descendant_with_program, inspect_forbidden_process_table,
    process_table_with_child, process_table_with_raw_child,
};
mod npm_probe;
mod process_scan;
mod process_table;
mod shim_security;
