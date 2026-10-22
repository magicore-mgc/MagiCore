//! Owns stable v3 schema and dependency ownership vocabulary.
//! Sở hữu version schema v3 và từ vựng quyền sở hữu dependency ổn định.

/// Canonical schema version emitted by this crate — Phiên bản schema canonical mà crate này ghi ra.
pub const LOCKFILE_SCHEMA_VERSION: &str = "3";

/// Provenance source kind: package pin imported from an ecosystem lockfile.
/// Nguồn gốc: pin package được nhập từ lockfile của hệ sinh thái.
pub const SOURCE_KIND_REGISTRY_IMPORT: &str = "registry-import";
/// Provenance source kind: resolved natively by the mgc resolver (Phase 2).
/// Nguồn gốc: resolve nội bộ bởi resolver của mgc (Phase 2).
pub const SOURCE_KIND_NATIVE_RESOLVE: &str = "native-resolve";
/// Provenance source kind: delegated to the ecosystem toolchain (audit-led).
/// Nguồn gốc: ủy thác cho toolchain hệ sinh thái (có audit).
pub const SOURCE_KIND_DELEGATED_TOOL: &str = "delegated-tool";

/// Dependency ownership: mgc resolves the package natively.
/// Quyền sở phụ thuộc: mgc tự resolve.
pub const OWNER_MGC_NATIVE: &str = "mgc-native";
/// Dependency ownership: delegated to the ecosystem toolchain.
/// Quyền sở phụ thuộc: ủy thác cho toolchain hệ sinh thái.
pub const OWNER_DELEGATED: &str = "delegated";
/// Dependency ownership: scaffold generated once, then user-owned.
/// Quyền sở phụ thuộc: scaffold sinh một lần, sau đó user sở hữu.
pub const OWNER_SCAFFOLD_ONLY: &str = "scaffold-only";
/// Dependency ownership: no resolution path for this ecosystem yet.
/// Quyền sở phụ thuộc: chưa có đường resolve cho hệ sinh thái này.
pub const OWNER_UNSUPPORTED: &str = "unsupported";
