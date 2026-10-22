//! Builds an unconstrained manifest for a latest-version query.
//! Tạo manifest không giới hạn để query phiên bản mới nhất.

pub(in crate::commands::outdated::native) fn latest_candidate_manifest(
    manifest: &mgc_types::Manifest,
) -> mgc_types::Manifest {
    let mut latest = manifest.clone();
    let mut dependencies = Vec::new();
    for group in [
        &mut latest.dependencies,
        &mut latest.dev_dependencies,
        &mut latest.peer_dependencies,
        &mut latest.optional_dependencies,
    ] {
        for mut dependency in std::mem::take(group) {
            dependency.range = mgc_types::VersionRange::star();
            dependency.dev = false;
            dependency.optional = false;
            dependency.peer = false;
            dependencies.push(dependency);
        }
    }
    // Resolve each package name once even if it appears in several manifest groups.
    // Resolve mỗi package một lần kể cả khi tên xuất hiện ở nhiều nhóm manifest.
    dependencies.sort_by(|left, right| left.name.as_str().cmp(right.name.as_str()));
    dependencies.dedup_by(|left, right| left.name == right.name);
    latest.dependencies = dependencies;
    latest
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
