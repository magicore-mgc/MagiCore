//! Shared-cache state test and fixture wiring.
//! Wiring test trạng thái shared cache và fixture.

#![allow(clippy::unwrap_used, clippy::await_holding_lock)]

#[path = "startup_prune.rs"]
mod startup_prune;
#[path = "../../test/support.rs"]
#[allow(clippy::duplicate_mod)]
// Shared fixture source is compiled into each cache owner test module.
mod support;
