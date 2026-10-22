use super::super::{CrateIndexEntry, CratesProtocol, ResolvedEntry};
use super::{ResolveError, ResolveResult};
use mgc_types::Version;

pub(in crate::protocols::crates) async fn resolve(
    protocol: &CratesProtocol,
    name: &str,
    range: &str,
) -> ResolveResult<ResolvedEntry> {
    let index_url = super::super::index::index_url_for(protocol, name);
    let body = super::super::http::get_text(protocol, &index_url).await?;

    let mut best: Option<(Version, CrateIndexEntry)> = None;
    let mut best_yanked: Option<(Version, CrateIndexEntry)> = None;

    for line in body.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        let entry: CrateIndexEntry =
            serde_json::from_str(line).map_err(ResolveError::ParseEntry)?;
        if !entry.name.eq_ignore_ascii_case(name) {
            continue;
        }
        let Ok(version) = Version::parse(&entry.vers) else {
            continue;
        };
        if !super::super::version::cargo_matches(range, &version) {
            continue;
        }
        if entry.yanked {
            if best_yanked.as_ref().is_none_or(|(v, _)| version > *v) {
                best_yanked = Some((version, entry));
            }
            continue;
        }
        if best.as_ref().is_none_or(|(v, _)| version > *v) {
            best = Some((version, entry));
        }
    }

    let chosen = best.or_else(|| {
        // Never select a yanked version; if only yanked candidates
        // match, warn and fail closed (the dependency is skipped by the
        // caller, never silently pinned to a yanked artifact).
        // Không bao giờ chọn version yanked; nếu chỉ còn ứng viên yanked
        // khớp, cảnh báo và fail-closed (caller bỏ dependency, không bao
        // giờ âm thầm ghim vào artifact yanked).
        if best_yanked.is_some() {
            eprintln!(
                "WARNING: {name}@{range} only matches yanked versions — skipping (fail-closed)"
            );
        }
        None
    });

    let entry = chosen
        .map(|(_, e)| e)
        .ok_or_else(|| ResolveError::NoMatchingVersion {
            name: name.to_string(),
            range: range.to_string(),
        })?;

    let artifact_url = super::super::index::download_url_for(protocol, name, &entry.vers);
    // The REAL sparse index carries a BARE 64-hex `cksum` (no
    // scheme prefix) — an earlier revision only accepted a
    // `sha256:`-prefixed value and dropped every genuine checksum.
    // (Index thật mang `cksum` hex trần, không prefix.)
    let sha256 = entry
        .cksum
        .as_deref()
        .map(str::trim)
        .filter(|c| c.len() == 64 && c.chars().all(|ch| ch.is_ascii_hexdigit()))
        .or_else(|| {
            entry
                .cksum
                .as_deref()
                .and_then(|c| c.strip_prefix("sha256:"))
                .filter(|c| c.len() == 64 && c.chars().all(|ch| ch.is_ascii_hexdigit()))
        })
        .map(|c| c.to_ascii_lowercase())
        .ok_or_else(|| ResolveError::InvalidChecksum {
            name: entry.name.clone(),
            version: entry.vers.clone(),
        })?;

    let mut resolved = ResolvedEntry {
        name: name.to_string(),
        version: entry.vers.clone(),
        deps: Vec::new(),
        artifact_url,
        sha256,
        extra_markers: Vec::new(),
    };

    for dep in &entry.deps {
        let kind = dep.kind.as_deref().unwrap_or("normal");
        // dev deps are excluded from the graph — only normal + build.
        // (dev deps bị loại khỏi graph — chỉ normal + build.)
        if kind == "dev" {
            continue;
        }
        let actual_name = dep.package.clone().unwrap_or_else(|| dep.name.clone());
        // Optional deps are not in the selected graph unless a feature
        // activates them. Keep an audit marker, but do not resolve or
        // download dormant packages.
        // (Optional dep chưa thuộc graph được chọn nếu feature chưa bật;
        // giữ marker audit nhưng không resolve/tải package đang ngủ.)
        if dep.optional {
            resolved
                .extra_markers
                .push(format!("optional:{actual_name}"));
            continue;
        }
        if let Some(registry) = &dep.registry {
            return Err(ResolveError::AlternateRegistry {
                package: name.to_string(),
                version: entry.vers.clone(),
                dependency: dep.name.clone(),
                registry: registry.clone(),
            });
        }
        // Cargo.lock is shared across target platforms. Include every
        // target-qualified edge so builds on another OS can stay locked;
        // the source Cargo.toml still decides whether Cargo compiles it.
        // (Cargo.lock dùng chung giữa các target; giữ mọi cạnh có điều
        // kiện để build OS khác vẫn locked; Cargo.toml quyết định compile.)
        if let Some(target) = &dep.target {
            resolved.extra_markers.push(format!("target:{target}"));
        }
        let req = if dep.req.trim().is_empty() {
            "*".to_string()
        } else {
            dep.req.clone()
        };
        resolved.deps.push((actual_name, req));
    }

    Ok(resolved)
}
