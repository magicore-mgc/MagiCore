//! Quota-prune test and support wiring.
//! Wiring test quota-prune và fixture.

#[path = "quota.rs"]
mod quota;
#[path = "../../test/support.rs"]
#[allow(clippy::duplicate_mod)]
// Shared fixture source is compiled into each cache-prune owner test module.
mod support;
