use super::super::{CrateIndexEntry, CratesProtocol};
use super::{IndexError, IndexResult};
use mgc_types::Version;

pub(in crate::protocols::crates) fn index_url_for(protocol: &CratesProtocol, name: &str) -> String {
    let name = name.to_ascii_lowercase();
    let prefix = super::super::version::sparse_prefix(&name);
    format!("{}/{prefix}/{name}", protocol.index_url)
}

pub(in crate::protocols::crates) fn download_url_for(
    protocol: &CratesProtocol,
    name: &str,
    version: &str,
) -> String {
    format!("{}/{name}/{version}/download", protocol.download_base)
}

pub(in crate::protocols::crates) async fn index_entries(
    protocol: &CratesProtocol,
    name: &str,
) -> IndexResult<Vec<CrateIndexEntry>> {
    let body = super::super::http::get_text(protocol, &index_url_for(protocol, name)).await?;
    body.lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .map(|line| serde_json::from_str(line).map_err(IndexError::ParseEntry))
        .collect()
}

pub(in crate::protocols::crates) fn select_index_entry(
    name: &str,
    ranges: &[String],
    entries: Vec<CrateIndexEntry>,
) -> IndexResult<CrateIndexEntry> {
    let mut best: Option<(Version, CrateIndexEntry)> = None;
    let mut best_yanked: Option<(Version, CrateIndexEntry)> = None;
    for entry in entries {
        if !entry.name.eq_ignore_ascii_case(name) {
            continue;
        }
        let Ok(version) = Version::parse(&entry.vers) else {
            continue;
        };
        if !ranges
            .iter()
            .all(|range| super::super::version::cargo_matches(range, &version))
        {
            continue;
        }
        let slot = if entry.yanked {
            &mut best_yanked
        } else {
            &mut best
        };
        if slot.as_ref().is_none_or(|(current, _)| version > *current) {
            *slot = Some((version, entry));
        }
    }
    let chosen = best.or_else(|| {
        if best_yanked.is_some() {
            eprintln!(
                "WARNING: {name}@{} only matches yanked versions — refusing to resolve",
                ranges.join(", ")
            );
        }
        None
    });
    let (_, entry) = chosen.ok_or_else(|| IndexError::NoMatchingVersion {
        name: name.to_string(),
        ranges: ranges.join(", "),
    })?;
    let checksum = entry
        .cksum
        .as_deref()
        .map(str::trim)
        .filter(|value| value.len() == 64 && value.bytes().all(|byte| byte.is_ascii_hexdigit()))
        .or_else(|| {
            entry
                .cksum
                .as_deref()
                .and_then(|value| value.strip_prefix("sha256:"))
                .filter(|value| {
                    value.len() == 64 && value.bytes().all(|byte| byte.is_ascii_hexdigit())
                })
        })
        .map(str::to_ascii_lowercase)
        .ok_or_else(|| IndexError::InvalidChecksum {
            name: entry.name.clone(),
            version: entry.vers.clone(),
        })?;
    let mut entry = entry;
    entry.cksum = Some(checksum);
    Ok(entry)
}
