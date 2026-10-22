//! Merge trust roots and resolve the fail-closed unsigned policy.
//! Gộp trust root và phân giải chính sách unsigned theo fail-closed.

use crate::commands::self_update::error::SelfUpdateError;
use anyhow::Result;

/// Trust roots for manifest signatures: explicit `--trust-root` flags
/// (repeatable, hex Ed25519 pubkeys) plus `MGC_RELEASE_TRUST_ROOTS`
/// (comma-separated). Empty = no provenance verification (loud warning,
/// sha256-only — the transitional level, never claimed as verified).
/// (Trust roots cho chữ ký manifest.)
pub fn trust_roots(extra: &[String]) -> Vec<String> {
    merge_roots(
        extra,
        std::env::var("MGC_RELEASE_TRUST_ROOTS").ok().as_deref(),
    )
}

/// Pure merge (unit-tested; env reading stays at the boundary).
/// (Gộp thuần để test được.)
pub(in crate::commands) fn merge_roots(extra: &[String], env: Option<&str>) -> Vec<String> {
    let mut roots: Vec<String> = extra.to_vec();
    if let Some(env) = env {
        roots.extend(
            env.split(',')
                .map(str::trim)
                .filter(|s| !s.is_empty())
                .map(str::to_string),
        );
    }
    roots
}

/// Resolved provenance policy (P0/F2 — fail-closed):
/// - trust roots configured → Verified. Any unsigned downgrade fails,
///   and combining `--allow-unsigned` with roots is refused outright
///   (a pinned user must never silently stop verifying).
/// - no roots + explicit `--allow-unsigned` → UnsignedWarn: the single
///   escape hatch for legacy-unsigned releases, always loud.
/// - no roots, no flag → hard error prompting a choice (configure a
///   trust root, or pass `--allow-unsigned` alone).
///
/// (Chính sách provenance fail-closed: có root → Verified; không root +
/// cờ tường minh → UnsignedWarn ồn ào; còn lại lỗi cứng.)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProvenanceMode {
    Verified,
    UnsignedWarn,
}

pub fn resolve_provenance_mode(roots: &[String], allow_unsigned: bool) -> Result<ProvenanceMode> {
    if !roots.is_empty() && allow_unsigned {
        return Err(SelfUpdateError::unsigned_refused(
            "trust roots are configured — refusing --allow-unsigned (unset --trust-root / MGC_RELEASE_TRUST_ROOTS to update from a legacy-unsigned release)",
        ));
    }
    if !roots.is_empty() {
        return Ok(ProvenanceMode::Verified);
    }
    if allow_unsigned {
        return Ok(ProvenanceMode::UnsignedWarn);
    }
    Err(SelfUpdateError::unsigned_refused(
        "no trust roots configured and --allow-unsigned not passed — refusing sha256-only update (pin a key with --trust-root, or pass --allow-unsigned alone for a legacy-unsigned release)",
    ))
}
