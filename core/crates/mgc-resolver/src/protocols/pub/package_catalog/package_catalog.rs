//! Parse, validate and select pub.dev package catalog entries.
//! Parse, kiểm tra và chọn entry catalog package pub.dev.

use super::super::version_constraints::dart_matches;
use super::dependency_metadata::parse_pub_dependencies;
use super::error::CatalogError;
use super::types::{PubPackage, PubVersion};
use crate::protocols::ResolvedEntry;
use mgc_types::{MgResult, Version};

pub(in crate::protocols::pubdev) fn select_entry(
    name: &str,
    doc: &PubPackage,
    constraints: &[String],
) -> MgResult<ResolvedEntry> {
    let mut best: Option<(Version, &PubVersion)> = None;
    for candidate in &doc.versions {
        let Ok(version) = Version::parse(&candidate.version) else {
            continue;
        };
        if !constraints
            .iter()
            .all(|constraint| dart_matches(constraint, &version))
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
        CatalogError::NoMatchingVersion {
            name: name.to_string(),
            constraints: constraints.join(" AND "),
        }
        .into_mg_error()
    })?;
    if candidate.archive_url.is_empty() {
        return Err(CatalogError::MissingArchiveUrl {
            name: name.to_string(),
            version: version.to_string(),
        }
        .into_mg_error());
    }
    let archive_sha256 = candidate
        .archive_sha256
        .as_deref()
        .map(str::trim)
        .filter(|digest| digest.len() == 64 && digest.chars().all(|ch| ch.is_ascii_hexdigit()))
        .ok_or_else(|| {
            CatalogError::MissingArchiveDigest {
                name: name.to_string(),
                version: version.to_string(),
            }
            .into_mg_error()
        })?
        .to_ascii_lowercase();

    let mut markers = Vec::new();
    let pubspec = candidate.pubspec.as_ref().ok_or_else(|| {
        CatalogError::MissingPubspec {
            name: name.to_string(),
            version: version.to_string(),
        }
        .into_mg_error()
    })?;
    if let Some(environment) = &pubspec.environment
        && !environment.sdk.is_empty()
    {
        markers.push(format!("sdk:{}", environment.sdk));
    }
    let deps = match &pubspec.dependencies {
        // Pubspecs without a dependency section have no registry edges.
        // A null field in pub.dev's JSON representation has the same
        // meaning as the omitted section; a null whole-pubspec is rejected
        // above because its graph cannot be proven.
        None => Vec::new(),
        Some(dependencies) => parse_pub_dependencies(dependencies, &mut markers)?,
    };
    Ok(ResolvedEntry {
        name: name.to_string(),
        version: version.to_string(),
        deps,
        artifact_url: candidate.archive_url.clone(),
        sha256: archive_sha256,
        extra_markers: markers,
    })
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
