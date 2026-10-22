//! Source checksum behavior owners.
//! Các owner hành vi checksum source.

#[path = "canonical_file_map/mod.rs"]
mod canonical_file_map;
#[path = "tree_hash/mod.rs"]
mod tree_hash;
#[path = "validation/mod.rs"]
mod validation;
#[path = "verify_record/mod.rs"]
mod verify_record;
#[path = "write_record/mod.rs"]
mod write_record;

pub(in crate::install::cargo_compat) use validation::validate_sha256;
pub(crate) use verify_record::verify_source_checksum;
pub(crate) use write_record::write_source_checksum;
