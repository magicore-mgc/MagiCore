//! Shared environment doctor command owners.
//! Các owner dùng chung cho lệnh kiểm tra môi trường.

mod analysis;
mod command;
mod doctor;
mod error;
mod low_disk_space;
mod model;
mod presentation;
mod registry;
mod storage;
mod toolchain;

pub use command::DoctorCmd;
pub use doctor::handle;
pub use model::{DiagnosticIssue, DoctorReport, RegistryStatus, Tool};
