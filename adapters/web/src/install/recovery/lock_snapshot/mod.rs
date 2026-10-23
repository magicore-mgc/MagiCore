//! Lock snapshot owner.
//! Owner snapshot lock.

mod lock_snapshot;

pub(crate) use lock_snapshot::{
    restore_prior_lock_result, snapshot_prior_lock, sweep_stale_snapshots,
};
