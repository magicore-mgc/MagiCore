//! Exports the stable schema-v3 public model and schema vocabulary.
//! Export model công khai schema-v3 và từ vựng schema ổn định.

mod artifact_ref;
mod lockfile;
mod ownership_ledger;
mod package;
mod provenance;
mod schema;
mod signature_file;
mod workspace_topology;

pub use self::artifact_ref::ArtifactRef;
pub use self::lockfile::{Lockfile, LockfileMetadata, SignerInfo};
pub use self::ownership_ledger::{OwnershipEntry, load_ownership_ledger};
pub use self::package::Package;
pub use self::provenance::Provenance;
pub use self::schema::{
    LOCKFILE_SCHEMA_VERSION, OWNER_DELEGATED, OWNER_MGC_NATIVE, OWNER_SCAFFOLD_ONLY,
    OWNER_UNSUPPORTED, SOURCE_KIND_DELEGATED_TOOL, SOURCE_KIND_NATIVE_RESOLVE,
    SOURCE_KIND_REGISTRY_IMPORT,
};
pub use self::signature_file::SignatureFile;
pub use self::workspace_topology::{CrossEdge, WorkspaceTopology};
