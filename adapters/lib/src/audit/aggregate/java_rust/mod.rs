//! Wiring for the Java + Rust audit plan.
//! Wiring cho plan audit Java + Rust.

mod java_rust;
pub(in crate::audit) use java_rust::aggregate_java_rust;
