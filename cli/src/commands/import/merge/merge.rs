//! Merges imported pins while preserving other core owners.
//! Gộp pin import nhưng vẫn giữ package thuộc core khác.

/// Merge a legacy import into the unified lock without discarding pins owned
/// by another core. Ownerless entries are adopted only when the existing
/// document has no competing explicit owner; otherwise they remain untouched
/// because their origin cannot be proven.
/// (Trộn lock import mà không xóa pin core khác; entry cũ mơ hồ chỉ nhận khi
/// không có owner cạnh tranh.)
pub(crate) fn merge_imported_lock(
    mut imported: mgc_lockfile::Lockfile,
    existing: Option<mgc_lockfile::Lockfile>,
    owner_core: &str,
) -> mgc_lockfile::Lockfile {
    for package in &mut imported.packages {
        package.owner_core = Some(owner_core.to_string());
        // Supported legacy files handled here describe npm-compatible JS
        // graphs. Deno's JSR entries are reported as skipped by the parser;
        // imported npm records must not remain `other` and leak into another
        // ecosystem's install graph.
        // (Các lock legacy này mô tả graph JS/npm; không để entry rơi vào `other`.)
        package.ecosystem = mgc_lockfile::EcosystemTag::Web;
    }
    let Some(mut existing) = existing else {
        return imported;
    };

    let has_competing_owner = existing.packages.iter().any(|package| {
        package
            .owner_core
            .as_deref()
            .is_some_and(|owner| owner != owner_core)
    });
    if !has_competing_owner {
        for package in &mut existing.packages {
            if package.owner_core.is_none() {
                package.owner_core = Some(owner_core.to_string());
            }
        }
    }
    existing.packages.retain(|package| {
        package.owner_core.as_deref() != Some(owner_core)
            || !matches!(
                package.ecosystem,
                mgc_lockfile::EcosystemTag::Other | mgc_lockfile::EcosystemTag::Web
            )
    });
    existing.packages.extend(imported.packages);
    existing
        .root_dependencies
        .extend(imported.root_dependencies);
    existing.root_dependencies.sort();
    existing.root_dependencies.dedup();
    existing.version = mgc_lockfile::LOCKFILE_SCHEMA_VERSION.to_string();
    existing.metadata.generated_at = chrono::Utc::now().to_rfc3339();
    existing.metadata.generator = format!("mgc/{}", env!("CARGO_PKG_VERSION"));
    existing
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
