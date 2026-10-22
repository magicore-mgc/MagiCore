#![cfg(test)]
#![allow(clippy::unwrap_used, clippy::await_holding_lock)]
#![allow(unsafe_code)]
//! Unit-test facade — include the unit test implementation in the crate test scope.
//! Facade unit test — include implementation vào scope test của crate.

include!("unit_tests.rs");
