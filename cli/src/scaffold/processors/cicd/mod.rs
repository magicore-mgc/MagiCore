//! CI/CD scaffold owner exports.
//! Export của owner scaffold CI/CD.

#[path = "cicd.rs"]
mod implementation;
pub use implementation::CicdProcessor;
