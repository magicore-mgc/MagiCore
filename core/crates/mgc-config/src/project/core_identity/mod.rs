//! core identity module wiring and exports.
//! Wiring và export module core identity.
mod core_identity;
pub use core_identity::{
    ProjectIdentitySnapshot, project_identity_matches_snapshot, restore_project_identity,
    snapshot_project_identity,
};
