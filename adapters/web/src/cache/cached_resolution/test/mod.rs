//! Cached-resolution tests and shared fixture wiring.
//! Wiring test và fixture cho cached resolution.

#![allow(clippy::unwrap_used, clippy::await_holding_lock)]

#[path = "cache_reuse.rs"]
mod cache_reuse;
#[path = "../../test/support.rs"]
#[allow(clippy::duplicate_mod)]
// Shared fixture source is compiled into each cache owner test module.
mod support;
