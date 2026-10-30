//! Web install owners and stable public routes.
//! Các owner Web install và route public ổn định.

pub mod bin;
pub mod debug_trace;
pub mod download;
pub mod extract;
pub mod fetch;
pub mod integrity;
#[path = "link_tree/mod.rs"]
pub mod link_tree;
pub mod materialize;
pub mod package_marker;
pub mod script_policy;

mod recovery;
mod run_install;
mod tree_backup;

pub use crate::install::script_policy::{
    lifecycle_scripts_allowed, load_file_scripts_policy, load_trust_policies,
    should_run_lifecycle_scripts, trust_allows_script,
};
#[cfg(test)]
pub(crate) use recovery::sweep_stale_snapshots;
pub(crate) use recovery::{
    drain_lifecycle_tasks, recover_interrupted_install, restore_prior_lock_result,
    snapshot_prior_lock,
};
pub use run_install::{run_install, run_install_with_registry};
pub(crate) use tree_backup::TreeBackup;
