//! Serialize the exact deterministic v4 signed payload.
//! Serialize payload v4 được ký với byte tất định chính xác.

use crate::canonical::{
    LockfilePayload, package_section::append_package_section,
    payload_header::append_payload_header, peer_context_section::append_peer_context_section,
    root_dependencies_section::append_root_dependencies_section,
    workspace_section::append_workspace_section,
};

/// Deterministic TOML serialization of the canonical payload — the ONLY
/// bytes the digest covers. Fixed section order, sorted collections,
/// `\n` endings on every platform (including Windows).
///
/// Precondition: integer fields fit i64 (TOML integers are i64-only).
/// Violations emit TOML that fails to parse back, never silently wrong bytes.
/// Serialize TOML tất định của payload canonical — byte DUY NHẤT digest phủ.
pub fn canonical_toml(payload: &LockfilePayload) -> String {
    let mut out = String::new();
    append_payload_header(&mut out, payload);
    append_peer_context_section(&mut out, payload);
    append_root_dependencies_section(&mut out, payload);
    append_workspace_section(&mut out, payload);
    append_package_section(&mut out, payload);
    out
}
