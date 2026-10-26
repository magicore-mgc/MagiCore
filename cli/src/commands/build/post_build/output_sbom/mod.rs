//! Output SBOM owner.
//! Owner SBOM output.

#[path = "output_sbom.rs"]
mod implementation;

pub(crate) use implementation::write;
#[cfg(test)]
pub(crate) use implementation::{OutputFile, hash_output};
