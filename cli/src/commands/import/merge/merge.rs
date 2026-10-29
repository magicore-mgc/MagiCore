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
    // Drop roots that no longer have a package. The `retain` above replaces the
    // previous Web package set with the freshly imported one, so a root recorded
    // by an earlier import can point at a package this merge removed — leaving
    // `root_dependencies = ["nanoid@5.1.16"]` with no `nanoid` entry. `mgc why`
    // then reports the package as "not pinned", and `freshness` fails on a lock
    // that validates structurally but references a phantom.
    // (Bỏ root không còn package. `retain` ở trên thay bộ package Web cũ bằng bộ
    // vừa import, nên root của lần import trước có thể trỏ tới package đã bị xoá.)
    // Prune the MERGED set, not just `existing`: a root the incoming lock brought
    // in can also be a phantom, and pruning only `existing` left it forever.
    // (Dọn trên tập ĐÃ MERGE, không chỉ `existing` — root từ lock mới cũng có thể
    // là ma và sẽ tồn tại mãi nếu chỉ dọn `existing`.)
    let available: std::collections::BTreeSet<(String, String)> = existing
        .packages
        .iter()
        .map(|package| (package.name.clone(), package.version.clone()))
        .collect();
    let keeps_root = |root: &String| -> bool {
        let parsed = mgc_lockfile::root_pin::parse_root_pin(root);
        // A root owned by a DIFFERENT ecosystem is not this merge's business: the
        // retain above only replaced the Web package set.
        // (Root của ecosystem khác không thuộc phạm vi merge này.)
        if parsed
            .ecosystem
            .is_some_and(|ecosystem| ecosystem != mgc_lockfile::EcosystemTag::Web)
        {
            return true;
        }
        // `parse_root_pin` strips the `mgc-root-v1:<eco>:` prefix, so splitting at
        // the LAST `@` is correct for scoped names (`@scope/name@1.0.0`) — plain
        // `rsplit_once('@')` on a bare scoped root produced `("", "scope/name")`
        // and dropped the root.
        // (`parse_root_pin` bỏ prefix; tách tại `@` CUỐI đúng cho tên scoped —
        // `rsplit_once('@')` trên root scoped trần cho `("", "scope/name")` và làm
        // mất root.)
        let Some((name, version)) = parsed.package_id.rsplit_once('@') else {
            // A legacy unversioned root cannot be matched; keep it rather than
            // silently dropping a record whose shape we do not model.
            // (Root cũ không version giữ nguyên.)
            return true;
        };
        if name.is_empty() {
            return true;
        }
        available.contains(&(name.to_string(), version.to_string()))
    };
    existing.root_dependencies.retain(keeps_root);
    existing.version = mgc_lockfile::LOCKFILE_SCHEMA_VERSION.to_string();
    existing.metadata.generated_at = chrono::Utc::now().to_rfc3339();
    existing.metadata.generator = format!("mgc/{}", env!("CARGO_PKG_VERSION"));
    existing
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
