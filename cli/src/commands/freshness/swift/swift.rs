//! Owns Swift package identity projection and its existing app freshness gate. — Sở hữu projection identity package Swift và app freshness gate hiện có.

use std::path::Path;

use anyhow::Result;

use super::super::manifest_file::check_manifest_file;

/// Extract Swift package identities matching adapter normalization
/// (lowercase; registry `id: "scope.name"` and git `url:` repo tails —
/// see `adapters/app/src/manifest/swift/swift.rs`).
/// (Trích identity package Swift khớp chuẩn hóa adapter (lowercase;
/// `id: "scope.name"` registry và đuôi repo `url:` git; owner ở đường dẫn trên.)
#[allow(dead_code)]
pub fn declared_swift_deps(package_swift: &str) -> Vec<String> {
    let mut names = Vec::new();
    let mut rest = package_swift;
    while let Some(start) = rest.find(".package(") {
        rest = &rest[start + ".package(".len()..];
        let end = rest.find(')').unwrap_or(rest.len());
        let call = &rest[..end];
        // Registry form first: `.package(id: "scope.name", ...)`.
        // (Dạng registry trước: `.package(id: "scope.name", ...)`.)
        let identity = call
            .split("id:")
            .nth(1)
            .and_then(|s| {
                s.trim_start()
                    .strip_prefix('"')
                    .and_then(|s| s.split('"').next())
                    .map(str::to_string)
            })
            .or_else(|| {
                call.split("url:").nth(1).and_then(|s| {
                    let s = s.trim_start();
                    s.strip_prefix('"')
                        .and_then(|s| s.split('"').next())
                        .map(|url| {
                            url.trim_end_matches('/')
                                .trim_end_matches(".git")
                                .rsplit('/')
                                .next()
                                .unwrap_or_default()
                                .to_string()
                        })
                })
            });
        if let Some(identity) = identity {
            let identity = identity.to_lowercase();
            if !identity.is_empty() {
                names.push(identity);
            }
        }
        rest = &rest[end.min(rest.len())..];
    }
    names.sort();
    names.dedup();
    names
}
/// Swift freshness for app/dev flows (`Package.swift`).
/// Freshness Swift cho luồng app/dev (`Package.swift`).
///
/// Only meaningful where the Swift lane writes `mgc.lock` natively;
/// registry-vs-git identity mismatches fail closed with an install hint.
/// (Chỉ có nghĩa nơi lane Swift ghi `mgc.lock` native; lệch identity
/// registry-vs-git fail-closed kèm hướng install.)
pub fn check_swift_freshness(project_root: &Path) -> Result<()> {
    check_manifest_file(
        project_root,
        "Package.swift",
        true,
        "app",
        mgc_lockfile::EcosystemTag::Swift,
        |content| Ok(declared_swift_deps(content)),
    )?;
    Ok(())
}
