//! AWS CodeBuild output owner wiring.
//! Wiring owner output AWS CodeBuild.

#[path = "aws.rs"]
mod implementation;
pub(super) use implementation::write;
