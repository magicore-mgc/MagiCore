//! Expose v4 revision compatibility and established schema routes.
//! Công bố tương thích revision v4 và route schema hiện hành.

/// Current additive v4 schema revision — Revision cộng thêm hiện tại của schema v4.
pub const LOCKFILE_SCHEMA_V4: &str = "4.1";

/// Original v4 revision remains readable — Revision v4 ban đầu vẫn đọc được.
pub const LOCKFILE_SCHEMA_V4_LEGACY: &str = "4";

/// Whether a declared format version belongs to the supported v4 family.
/// Version có thuộc nhóm format v4 mà reader hiện tại hỗ trợ không.
pub fn is_v4_schema_version(version: &str) -> bool {
    version == LOCKFILE_SCHEMA_V4_LEGACY || version == LOCKFILE_SCHEMA_V4
}

pub use super::canonical_identity::{canonical_name, canonical_version, peer_context_digest};
pub use super::dependency_edge::{Edge, EdgeKind, EdgeOrigin};
pub use super::package_identity::{PackageKey, VariantKey};
pub use super::signature_block::SignatureBlock;
pub use super::source_claim::{claim_matches, claim_specificity};
pub use super::source_ref::SourceRef;
pub use super::source_selection::{SourceSelectionError, UNKNOWN_SOURCE_PRIORITY, select_source};
pub use super::target_tuple::TargetTuple;

#[cfg(test)]
#[path = "test/v4.rs"]
mod tests;
