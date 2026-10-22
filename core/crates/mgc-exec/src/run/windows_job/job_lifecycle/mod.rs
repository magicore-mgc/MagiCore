//! Job Object creation, assignment, containment and termination facade.
//! Facade tạo Job Object, gắn process, duy trì containment và terminate.

#[path = "job_lifecycle.rs"]
mod implementation;

#[cfg(windows)]
pub(in crate::run::implementation) use implementation::ProcessTreeGuard;
