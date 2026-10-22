//! Stable Terraform provider-lock scanner routes.
//! Các route scanner provider-lock Terraform ổn định.

mod error;
mod parser;
mod report;
mod terraform;
mod types;

pub use terraform::{audit_terraform_lock, parse_terraform_lock};
