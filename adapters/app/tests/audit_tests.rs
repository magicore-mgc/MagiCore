#![cfg(test)]
#![allow(clippy::unwrap_used)]

//! App audit integration-test target wiring.
//! Nối target integration test audit App.

#[path = "audit_tests/mod.rs"]
mod audit_tests;
