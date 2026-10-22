//! Select compatible solved candidates and detect exact version constraints.
//! Chọn candidate đã solve tương thích và nhận diện constraint version chính xác.

use super::super::resolved_manifest::is_manifest_root_package;
use mgc_types::{Manifest, Version, VersionRange};

/// Prefer a compatible project root when binding a package dependency edge.
/// Ưu tiên project root tương thích khi gắn dependency edge.
pub fn select_resolved_candidate<'a>(
    candidates: &'a [&'a mgc_resolver::solver::Resolution],
    constraint: &VersionRange,
    manifest: &Manifest,
) -> Option<&'a mgc_resolver::solver::Resolution> {
    candidates
        .iter()
        .copied()
        .filter(|candidate| constraint.matches(candidate.package_id.version()))
        .filter(|candidate| is_manifest_root_package(manifest, &candidate.package_id))
        .max_by(|left, right| left.package_id.version().cmp(right.package_id.version()))
        .or_else(|| {
            candidates
                .iter()
                .copied()
                .filter(|candidate| constraint.matches(candidate.package_id.version()))
                .max_by(|left, right| left.package_id.version().cmp(right.package_id.version()))
        })
}

/// Return a package version only when a range denotes one exact version.
/// A range's satisfying lower bound is not evidence that the registry chose it.
/// Chỉ trả version khi range chỉ rõ đúng một version; lower bound không chứng
/// minh registry đã chọn version đó.
pub fn exact_version_from_range(range: &VersionRange) -> Option<Version> {
    let raw = range.as_str().trim();
    let exact = raw.strip_prefix('=').unwrap_or(raw).trim();
    if exact.is_empty()
        || exact
            .chars()
            .next()
            .is_some_and(|ch| matches!(ch, '^' | '~' | '>' | '<'))
        || exact.contains("||")
    {
        return None;
    }
    Version::parse(exact)
        .ok()
        .filter(|version| range.matches(version))
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
