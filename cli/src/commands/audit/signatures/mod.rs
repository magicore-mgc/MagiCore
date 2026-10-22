//! Trusted-signature audit API and responsibility owners.
//! API audit chữ ký tin cậy và các owner theo trách nhiệm.

mod bundle_reader;
mod error;
mod package_scope;
mod registry_chain;
mod signatures;
mod sigstore_attestation;

pub use signatures::run;

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
