//! Shared, schema-aware lockfile writer facade.
//! Facade ghi lockfile dùng chung, có nhận biết schema.

mod content_equivalence;
mod legacy_signing;
mod mutation_policy;
mod serialize_v3;
mod v4_finalization;

pub(crate) use self::content_equivalence::same_v4_content;
pub use self::legacy_signing::{sign_and_write_lockfile, sign_lockfile_with_default_key};
pub use self::mutation_policy::ensure_lockfile_mutation_allowed;
pub use self::serialize_v3::{serialize_lockfile, write_lockfile};
pub use self::v4_finalization::finalize_v4_document;

pub(in crate::writer) use self::mutation_policy::ensure_not_v4_legacy_write;
pub(in crate::writer) use self::serialize_v3::write_lockfile_unchecked;
