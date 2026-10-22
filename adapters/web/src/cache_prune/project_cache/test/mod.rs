//! Project Cache tests.
//! Test project cache.

#[path = "cas_prune.rs"]
mod cas_prune;
#[path = "../../test/support.rs"]
#[allow(clippy::duplicate_mod)]
// Shared fixture source is compiled into each cache-prune owner test module.
mod support;
