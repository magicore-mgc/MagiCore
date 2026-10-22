//! Wire v4 schema and source-policy owners.
//! Nối owner schema v4 và policy source.

mod canonical_identity;
mod dependency_edge;
mod package_identity;
mod signature_block;
mod source_claim;
mod source_ref;
mod source_selection;
mod target_tuple;
mod v4;

pub use v4::*;
