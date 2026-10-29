//! Validate, select and project Packagist package catalog entries.
//! Kiểm tra, chọn và chiếu entry catalog package Packagist.

use super::super::version_constraints::composer_matches;
use super::dependency_metadata::{DependencyKind, parse_composer_dependencies};
use super::super::error::ComposerError;
use super::types::{PackagistDist, PackagistDocument, PackagistVersion};
use crate::protocols::ResolvedEntry;
use mgc_types::{MgError, MgResult, Version};

/// Marker recording the Composer `dist.reference` when no sha1 is published.
/// Marker ghi `dist.reference` của Composer khi registry không công bố sha1.
pub(in crate::protocols::composer) const DIST_REFERENCE_MARKER_PREFIX: &str = "dist-reference:";

/// Select the highest version satisfying every constraint and project it to
/// a [`ResolvedEntry`].
/// Chọn version cao nhất thoả mọi ràng buộc rồi chiếu sang [`ResolvedEntry`].
pub(in crate::protocols::composer) fn select_entry(
    name: &str,
    doc: &PackagistDocument,
    constraints: &[String],
) -> MgResult<ResolvedEntry> {
    let candidates = doc
        .packages
        .get(name)
        .ok_or_else(|| ComposerError::MissingPackage { name: name.to_string() }.into_mg_error())?;

    let mut best: Option<(Version, &PackagistVersion)> = None;
    for candidate in candidates {
        let Some(trimmed) = trim_stability_suffix(&candidate.version) else {
            continue;
        };
        let Ok(version) = Version::parse(trimmed) else {
            continue;
        };
        if !constraints
            .iter()
            .all(|constraint| composer_matches(constraint, &version))
        {
            continue;
        }
        if best
            .as_ref()
            .is_none_or(|(selected, _)| version > *selected)
        {
            best = Some((version, candidate));
        }
    }
    let (version, candidate) = best.ok_or_else(|| {
        ComposerError::NoMatchingVersion {
            name: name.to_string(),
            constraints: constraints.join(" AND "),
        }
        .into_mg_error()
    })?;
    let display_version = version.to_string();
    project_entry(name, &display_version, candidate, &mut Vec::new())
}

/// Project one Packagist version entry into a [`ResolvedEntry`].
/// `dev_deps` collects the development edges of the selected version so the
/// caller can expose them separately from the runtime graph.
/// Chiếu một entry version Packagist sang [`ResolvedEntry`]; `dev_deps` nhận
/// các cạnh development để caller tách khỏi graph runtime.
pub(in crate::protocols::composer) fn project_entry(
    name: &str,
    display_version: &str,
    candidate: &PackagistVersion,
    dev_deps: &mut Vec<(String, String)>,
) -> MgResult<ResolvedEntry> {
    let (dist_url, integrity) = dist_identity(name, display_version, candidate.dist.as_ref())?;
    let parsed = parse_composer_dependencies(&candidate.require, &candidate.require_dev);

    let mut deps = Vec::with_capacity(parsed.edges.len());
    let mut markers = parsed.platform_markers;
    for (kind, dep_name, constraint) in parsed.edges {
        if constraint.trim().is_empty() {
            return Err(ComposerError::EmptyConstraint { name: dep_name }.into_mg_error());
        }
        if kind == DependencyKind::Development {
            dev_deps.push((dep_name.clone(), constraint.clone()));
        }
        markers.push(format!(
            "{}{dep_name}:{constraint}",
            kind.marker_prefix()
        ));
        deps.push((format!("{}{dep_name}", kind.edge_prefix()), constraint));
    }
    for alias in parsed.substituted {
        markers.push(format!("substituted:{alias}"));
    }
    deps.sort();
    match integrity {
        DistIntegrity::Sha1(sha1) => markers.push(format!("checksum-sha1:{sha1}")),
        DistIntegrity::Reference(reference) => {
            markers.push(format!("{DIST_REFERENCE_MARKER_PREFIX}{reference}"))
        }
    }
    markers.sort();

    Ok(ResolvedEntry {
        name: name.to_string(),
        version: display_version.to_string(),
        deps,
        artifact_url: dist_url,
        sha256: String::new(),
        extra_markers: markers,
    })
}

/// Integrity source a Packagist dist entry vouches for.
/// Nguồn toàn vẹn mà entry dist Packagist bảo lãnh.
enum DistIntegrity {
    /// Published `dist.shasum` (sha1 hex).
    Sha1(String),
    /// VCS commit of the archive when no sha1 is published.
    Reference(String),
}

fn dist_identity(
    name: &str,
    version: &str,
    dist: Option<&PackagistDist>,
) -> MgResult<(String, DistIntegrity)> {
    let dist = dist.ok_or_else(|| {
        ComposerError::MissingDistUrl {
            name: name.to_string(),
            version: version.to_string(),
        }
        .into_mg_error()
    })?;
    if dist.url.trim().is_empty() {
        return Err(ComposerError::MissingDistUrl {
            name: name.to_string(),
            version: version.to_string(),
        }
        .into_mg_error());
    }
    if !matches!(dist.archive_type.trim(), "" | "zip") {
        return Err(MgError::Unsupported {
            core: "web",
            capability: "Composer dist archive type",
            guidance: format!(
                "Packagist package {name} {version} ships a '{}' archive; only zip dists are supported",
                dist.archive_type
            ),
        });
    }
    let shasum = dist
        .shasum
        .as_deref()
        .map(str::trim)
        .filter(|value| !value.is_empty());
    if let Some(shasum) = shasum {
        let lower = shasum.to_ascii_lowercase();
        if lower.len() != 40 || !lower.chars().all(|ch| ch.is_ascii_hexdigit()) {
            return Err(ComposerError::MalformedDistShasum {
                name: name.to_string(),
                version: version.to_string(),
                shasum: shasum.to_string(),
            }
            .into_mg_error());
        }
        return Ok((dist.url.clone(), DistIntegrity::Sha1(lower)));
    }
    let reference = dist
        .reference
        .as_deref()
        .map(str::trim)
        .filter(|value| !value.is_empty() && value.chars().all(|ch| ch.is_ascii_hexdigit()))
        .ok_or_else(|| {
            ComposerError::MissingDistIntegrity {
                name: name.to_string(),
                version: version.to_string(),
            }
            .into_mg_error()
        })?;
    Ok((dist.url.clone(), DistIntegrity::Reference(reference.to_string())))
}

/// Composer appends a stability suffix (`-dev`, `-beta1`, `-RC1`, `-patch1`,
/// `-stable`) to the version it reports; keep only the numeric/pre-release
/// head our version type understands.
/// Composer gắn hậu tố stability vào version công bố; giữ phần số/pre-release
/// mà kiểu version của ta hiểu.
fn trim_stability_suffix(raw: &str) -> Option<&str> {
    let head = raw.split('+').next()?.trim();
    for suffix in ["-dev", "-patch", "-stable"] {
        if let Some(base) = head.strip_suffix(suffix) {
            return (!base.is_empty()).then_some(base);
        }
    }
    for suffix in ["-beta", "-alpha", "-RC", "-rc"] {
        if let Some(index) = head.find(suffix) {
            let base = &head[..index];
            return (!base.is_empty()).then_some(base);
        }
    }
    Some(head)

#[cfg(test)]
#[path = "test/package_catalog.rs"]
mod tests;
