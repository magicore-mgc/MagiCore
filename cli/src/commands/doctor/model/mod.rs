//! Report data types and their serialization contract.
//! Kiểu dữ liệu report và contract serialization.

mod model;

pub use model::{DiagnosticIssue, DoctorReport, RegistryStatus, Tool};

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
