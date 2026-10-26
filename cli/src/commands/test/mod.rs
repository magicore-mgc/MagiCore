//! Test command facade — export the native project test command.
//! Facade lệnh test — export lệnh test project native.

#[path = "test.rs"]
mod implementation;
pub use implementation::test;
pub(crate) use implementation::test_at;
pub(crate) use implementation::test_runtime_for_project;
