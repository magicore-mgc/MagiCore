//! Wiring for the Python + Rust audit plan.
//! Wiring cho plan audit Python + Rust.

mod python_rust;
pub(in crate::audit) use python_rust::aggregate_python_rust;
