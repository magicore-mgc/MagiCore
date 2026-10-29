//! Validate, select and project Packagist package catalog entries.
//! Kiểm tra, chọn và chiếu entry catalog package Packagist.

use super::super::error::ComposerError;
use super::super::version_constraints::{ComposerVersion, composer_matches_composer};
use super::dependency_metadata::{DependencyKind, parse_composer_dependencies};
use super::types::{PackagistDist, PackagistDocument, PackagistVersion};
use crate::protocols::ResolvedEntry;
use mgc_types::{MgError, MgResult};

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
    let candidates = doc.packages.get(name).ok_or_else(|| {
        ComposerError::MissingPackage {
            name: name.to_string(),
        }
        .into_mg_error()
    })?;

    let mut best: Option<(ComposerVersion, &PackagistVersion)> = None;
    for candidate in candidates {
        let Some(version) = ComposerVersion::parse(&candidate.version) else {
            continue;
        };
        if !constraints
            .iter()
            .all(|constraint| composer_matches_composer(constraint, &version))
        {
            continue;
        }
        let better = match best.as_ref() {
            None => true,
            Some((selected, _)) => {
                (
                    version.major,
                    version.minor,
                    version.patch,
                    version.stability.is_none(),
                ) > (
                    selected.major,
                    selected.minor,
                    selected.patch,
                    selected.stability.is_none(),
                )
            }
        };
        if better {
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
    // The display version keeps the pre-release tag the registry published;
    // everything else comes from the normalised Composer version.
    // Version hiển thị giữ tag pre-release registry công bố; phần còn lại
    // lấy từ version Composer đã chuẩn hoá.
    let display_version = format!(
        "{}.{}.{}{}",
        version.major,
        version.minor,
        version.patch,
        version
            .stability
            .as_deref()
            .map(|tag| format!("-{}", tag.to_ascii_uppercase()))
            .unwrap_or_default()
    );
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
        // `deps` is the production graph the shared resolver walks, so
        // `require-dev` edges stay out of it and travel in a marker the
        // workspace reads when it installs dev dependencies.
        // `deps` là graph production mà resolver dùng chung duyệt, nên cạnh
        // `require-dev` không nằm trong đó mà đi qua marker để workspace đọc
        // khi cài dev dependency.
        if kind == DependencyKind::Development {
            dev_deps.push((dep_name.clone(), constraint.clone()));
            markers.push(format!("{}{dep_name}:{constraint}", kind.marker_prefix()));
            continue;
        }
        markers.push(format!("{}{dep_name}:{constraint}", kind.marker_prefix()));
        deps.push((dep_name, constraint));
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

/// GitHub publishes Composer zips through `api.github.com/.../zipball/<ref>`,
/// which answers 403 to clients without a browser User-Agent. The same commit
/// is served anonymously from `codeload.github.com/<owner>/<repo>/legacy.zip/<ref>`,
/// so GitHub zipball dists are rewritten to the codeload form.
/// GitHub phát dist composer qua `api.github.com/.../zipball/<ref>` và trả
/// 403 cho client không có User-Agent trình duyệt. Cùng commit được phục vụ
/// ẩn danh ở `codeload.github.com/<owner>/<repo>/legacy.zip/<ref>`, nên dist
/// zipball GitHub được đổi sang dạng codeload.
fn normalize_dist_url(url: &str) -> String {
    let rest = match url.strip_prefix("https://api.github.com/repos/") {
        Some(rest) => rest,
        None => return url.to_string(),
    };
    let (repo_path, reference) = match rest.split_once("/zipball/") {
        Some((repo_path, reference)) if !repo_path.is_empty() && !reference.is_empty() => {
            (repo_path, reference)
        }
        _ => return url.to_string(),
    };
    if repo_path.matches('/').count() != 1 {
        return url.to_string();
    }
    format!("https://codeload.github.com/{repo_path}/legacy.zip/{reference}")
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
    let artifact_url = normalize_dist_url(dist.url.trim());
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
        return Ok((artifact_url, DistIntegrity::Sha1(lower)));
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
    Ok((
        artifact_url,
        DistIntegrity::Reference(reference.to_string()),
    ))
}

#[cfg(test)]
#[path = "test/package_catalog.rs"]
mod tests;
