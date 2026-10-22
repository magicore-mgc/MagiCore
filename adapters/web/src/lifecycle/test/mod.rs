#![cfg(test)]
#![allow(clippy::unwrap_used)]
// Lifecycle tests live beside the guarded lifecycle owner.
// Test lifecycle được đặt cạnh owner lifecycle có kiểm soát.
#![allow(unsafe_code)]

mod environment;
mod error;
mod helpers;
mod manifest;
mod project_identity;
mod script_execution;
mod script_policy;
