//! Terraform lock parser owner routes.
//! Các route owner parser Terraform lock.

mod checksum_presence;
mod parser;
mod provider_state;

pub(super) use parser::parse;
