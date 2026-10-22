//! Install recovery owner wiring and nearest tests.
//! Wiring owner recovery install và test gần module.

mod recovery;
#[cfg(test)]
pub(crate) use recovery::sweep_stale_snapshots;
pub(crate) use recovery::{
    drain_lifecycle_tasks, recover_interrupted_install, restore_prior_lock_result,
    snapshot_prior_lock,
};

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
