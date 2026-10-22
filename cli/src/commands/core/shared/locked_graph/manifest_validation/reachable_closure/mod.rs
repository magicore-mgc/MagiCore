//! Reachable-closure owner wiring.
//! Wiring owner xác thực closure reachable.

#[path = "reachable_closure.rs"]
mod implementation;

pub(crate) use implementation::reachable_closure_matches;
