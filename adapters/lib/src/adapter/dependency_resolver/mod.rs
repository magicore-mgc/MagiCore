//! Native resolver trait facade, lane orchestration, and pending-lock staging.
//! Facade trait resolver native, điều phối lane và staging pending-lock.

mod dependency_resolver;
mod error;
mod lock_staging;
mod probe;
mod resolve;

pub(super) use lock_staging::stage_pending_lock;

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
