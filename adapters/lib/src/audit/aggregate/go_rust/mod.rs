//! Wiring for the Go + Rust audit plan.
//! Wiring cho plan audit Go + Rust.

mod go_rust;
pub(in crate::audit) use go_rust::aggregate_go_rust;
