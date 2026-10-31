//! Lockfile v4 canonical owner wiring and stable API exports.
//! File này chỉ nối owner canonical v4 và giữ nguyên API công khai.

mod artifact_ref;
mod canonical;
mod document_io;
mod edge_table;
mod package_section;
mod package_table;
mod payload_digest;
mod payload_header;
mod peer_context_section;
mod provenance;
mod revision_validation;
mod root_dependencies_section;
mod sort_key;
mod source_table;
mod toml_escaping;
mod toml_fields;
mod types;
mod workspace;
mod workspace_section;

pub(in crate::canonical) use artifact_ref::deserialize_v4_artifact;
pub(in crate::canonical) use provenance::deserialize_v4_provenance;
pub(in crate::canonical) use workspace::deserialize_v4_workspace;

pub use canonical::canonical_toml;
pub use document_io::{parse_v4_document, write_v4_document};
pub(crate) use package_table::push_package_key as package_key_table;
pub use payload_digest::payload_digest;
pub(crate) use revision_validation::validate_v4_revision;
pub use types::{LockfilePayload, LockfileV4, LockfileV4Metadata, PackageV4};
