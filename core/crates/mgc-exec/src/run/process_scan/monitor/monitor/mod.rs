//! Unix process monitor implementation wiring.
//! Wiring phần triển khai monitor process Unix.

#[cfg(unix)]
mod descendant_refresh;
#[cfg(unix)]
use descendant_refresh::refresh_live_descendant_pids;
#[cfg(unix)]
mod forbidden_selection;
#[cfg(unix)]
use forbidden_selection::find_first_forbidden;
#[cfg(unix)]
mod go_environment;
#[cfg(unix)]
use go_environment::refresh_go_processes;
#[cfg(unix)]
mod process_identity;
#[cfg(unix)]
use process_identity::{
    current_process_scan_snapshot, live_descendant_identities, process_identity_is_live,
};
#[cfg(unix)]
mod process_policy;
#[cfg(unix)]
use process_policy::forbidden_process_entry;
#[cfg(unix)]
mod process_snapshot;
#[cfg(unix)]
use process_snapshot::collect_process_snapshots;
#[cfg(unix)]
mod observed_groups;
#[cfg(unix)]
use observed_groups::refresh_live_observed_process_groups;
#[cfg(unix)]
mod root_exit;
#[cfg(unix)]
use root_exit::scan_after_root_exit;
#[cfg(unix)]
mod npm_probe;
#[cfg(unix)]
use npm_probe::is_allowed_shadow_npm_version_probe;
#[cfg(test)]
use npm_probe::{
    is_generated_shadow_npm_version_probe, is_shadow_npm_version_probe,
    npm_probe_command_line_matches_shim_path,
};
mod monitor;
#[cfg(unix)]
pub(in crate::run::implementation) use monitor::find_forbidden_descendant_tracking;
#[cfg(all(unix, test))]
use monitor::{find_forbidden_descendant, find_forbidden_descendant_with_scope};
#[cfg(all(unix, test))]
#[path = "../test/mod.rs"]
mod test;
