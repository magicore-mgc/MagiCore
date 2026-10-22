//! Manifest Digest implementation.
//! Implementation manifest digest của shared mutation.

use mgc_types::Manifest;

/// Canonical digest of a manifest for restore verification: project
/// name+version plus EVERY dependency with group, range and flags,
/// sorted. Name-only comparison would bless a restore that changed
/// versions, groups or project fields.
/// (Digest chuẩn manifest: tên+version project, mọi dep kèm group,
/// range, cờ — đã sắp xếp.)
pub(crate) fn manifest_canonical_digest(manifest: &Manifest) -> Vec<String> {
    let mut items = vec![format!(
        "project:{}@{}",
        manifest.name,
        manifest
            .version
            .as_ref()
            .map(|version| version.to_string())
            .unwrap_or_default()
    )];
    let mut deps: Vec<String> = [
        ("dep:", &manifest.dependencies),
        ("dev:", &manifest.dev_dependencies),
        ("peer:", &manifest.peer_dependencies),
        ("opt:", &manifest.optional_dependencies),
    ]
    .into_iter()
    .flat_map(|(group, specs)| {
        specs.iter().map(move |spec| {
            format!(
                "{group}{}@{}{}{}{}",
                spec.name.as_str(),
                spec.range.as_str(),
                if spec.dev { "+dev" } else { "" },
                if spec.optional { "+opt" } else { "" },
                if spec.peer { "+peer" } else { "" },
            )
        })
    })
    .collect();
    deps.sort();
    items.extend(deps);
    items
}
