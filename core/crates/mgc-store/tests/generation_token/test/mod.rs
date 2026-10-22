//! Behavior-focused generation-token test modules.
//! Module test generation-token được nhóm theo hành vi.

#[path = "allocation_races/mod.rs"]
pub(crate) mod allocation_races;
#[path = "claim_admission/mod.rs"]
pub(crate) mod claim_admission;
#[path = "install_coordination/mod.rs"]
pub(crate) mod install_coordination;
#[path = "integrity_snapshot/mod.rs"]
pub(crate) mod integrity_snapshot;
#[path = "migration/mod.rs"]
pub(crate) mod migration;
#[path = "promoted_coverage/mod.rs"]
pub(crate) mod promoted_coverage;
#[path = "staging_leases/mod.rs"]
pub(crate) mod staging_leases;
#[path = "token_transitions/mod.rs"]
pub(crate) mod token_transitions;
