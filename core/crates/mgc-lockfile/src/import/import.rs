//! Import legacy lockfiles — điều phối nhập lockfile cũ.

use crate::root_pin::format_root_pin;
use crate::schema::{Package, Provenance, SOURCE_KIND_REGISTRY_IMPORT};
use anyhow::{Result, bail};
use std::path::Path;

use super::{
    BUN_LOCKFILE, DENO_LOCKFILE, ImportReport, NPM_LOCKFILE, PNPM_LOCKFILE, SkippedRecord,
    YARN_LOCKFILE, detect_legacy_lockfiles, parse_bun, parse_deno, parse_npm, parse_pnpm,
    parse_yarn,
};

/// Detect theo độ ưu tiên npm > pnpm > yarn > bun rồi parse sang Lockfile v2.
/// Nhiều file cùng tồn tại → dùng file ưu tiên cao nhất (spec lockfile-import-plan §2).
// (Detect by priority npm > pnpm > yarn > bun and parse the first match.)
pub fn import_into_lockfile(project_root: &Path) -> Result<(crate::Lockfile, ImportReport)> {
    let candidates = detect_legacy_lockfiles(project_root);
    let first = candidates
        .first()
        .ok_or_else(|| anyhow::anyhow!("no supported legacy lockfile found (npm/pnpm/yarn/bun)"))?;
    import_file(&first.path)
}

/// Parse 1 file lockfile cụ thể → Lockfile v2 (chưa ký).
// (Parse one specific lockfile file into an unsigned v2 Lockfile.)
pub fn import_file(path: &Path) -> Result<(crate::Lockfile, ImportReport)> {
    let content = std::fs::read_to_string(path)
        .map_err(|e| anyhow::anyhow!("cannot read {}: {e}", path.display()))?;
    let file_name = path
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or_default();

    let mut warnings: Vec<String> = Vec::new();
    let mut skipped: Vec<SkippedRecord> = Vec::new();
    // Root direct-dep pins (P0 finding #6) — attached to the Lockfile
    // as the root graph, never as package self-edges.
    // Pin direct-dep gốc (P0 finding #6) — gắn vào Lockfile như graph
    // root, không bao giờ là self-edge của package.
    let mut root_deps: Vec<String> = Vec::new();
    let mut packages = match file_name {
        NPM_LOCKFILE => parse_npm(&content, &mut warnings, &mut root_deps)?,
        PNPM_LOCKFILE => parse_pnpm(&content, &mut warnings, &mut root_deps)?,
        // `yarn.lock` (classic) and `bun.lock` list every resolved package but do
        // not say which are DIRECT. Reading `package.json` beside the lock is the
        // only source for that, and without it the imported lock has no root graph:
        // every dependency looks transitive and `freshness` cannot check a
        // declared dependency.
        // (yarn.lock classic và bun.lock không nói package nào TRỰC TIẾP. Đọc
        // `package.json` cạnh lock là nguồn duy nhất; thiếu nó lock import không có
        // graph root.)
        YARN_LOCKFILE => {
            let packages = parse_yarn(&content)?;
            collect_manifest_roots(path, &packages, &mut root_deps, &mut warnings);
            packages
        }
        BUN_LOCKFILE => {
            let packages = parse_bun(&content, &mut skipped)?;
            collect_manifest_roots(path, &packages, &mut root_deps, &mut warnings);
            packages
        }
        DENO_LOCKFILE => parse_deno(&content, &mut skipped, &mut root_deps)?,
        other => bail!("unsupported lockfile '{other}'"),
    };

    if packages.is_empty() {
        bail!("no importable packages found in {file_name}");
    }

    // Preserve the source lockfile as provenance when it does not contain
    // registry URLs; never fabricate a registry endpoint from a package URL.
    // Ghi nguồn lockfile khi input không mang URL registry; không tự bịa URL.
    for package in &mut packages {
        package.provenance = Some(Provenance {
            source_kind: SOURCE_KIND_REGISTRY_IMPORT.to_string(),
            tool: None,
            imported_from: Some(file_name.to_string()),
        });
    }

    // Sắp xếp + khử trùng lặp (name,version) → output deterministic
    // (Sort + dedupe by (name, version) for deterministic output)
    packages.sort_by(|a, b| (&a.name, &a.version).cmp(&(&b.name, &b.version)));
    packages.dedup_by(|a, b| a.name == b.name && a.version == b.version);

    let count = packages.len();
    let mut lockfile = crate::Lockfile::new();
    lockfile.packages = packages;
    // Root graph (P0 finding #6): the source root pin set becomes the
    // lockfile root_dependencies — the honest representation of
    // "workspace root → dependency" edges.
    // Graph root (P0 finding #6): tập pin gốc của lockfile nguồn thành
    // root_dependencies — biểu diễn trung thực cạnh "workspace root →
    // dependency".
    lockfile.root_dependencies = root_deps;

    Ok((
        lockfile,
        ImportReport {
            source_file: file_name.to_string(),
            packages: count,
            warnings,
            skipped,
        },
    ))
}

#[cfg(test)]
#[path = "import/test/mod.rs"]
mod test;

/// Upper bound for the `package.json` read during import.
///
/// The same 10 MiB cap the lockfile itself uses: a manifest larger than that is
/// not a manifest, and reading it unbounded let a 96 MB file push peak RSS to
/// 221 MB.
/// (Cùng giới hạn 10 MiB như lockfile: manifest lớn hơn thế không phải manifest,
/// và đọc không giới hạn từng đẩy RSS lên 221 MB với file 96 MB.)
const MANIFEST_IMPORT_MAX_BYTES: u64 = 10 * 1024 * 1024;

/// Direct dependencies declared in `package.json` beside the lock, as
/// qualified root pins.
///
/// `yarn.lock` (classic) and `bun.lock` do not mark which entries the project
/// declares, so the manifest is the authority — but only for WHICH packages are
/// direct; the lock supplies their installed version. Emitting a bare name (the
/// previous shape) broke every consumer: `freshness` rejects a root without
/// `@version` (`stale_lock_needs_install`), and `merge_imported_lock` cannot match
/// it against a package.
///
/// The manifest is read through `read_bounded_regular_file`, which refuses a
/// symlink or a non-regular file and caps the size. A `package.json` symlinked to
/// a file outside the project previously fed that file's dependency names straight
/// into `mgc.lock`.
/// (`yarn.lock`/`bun.lock` không đánh dấu entry nào project khai, nên manifest
/// quyết định entry NÀO là trực tiếp; lock cấp version đã cài. Ghi tên trần như
/// trước làm hỏng mọi consumer. Manifest đọc qua `read_bounded_regular_file` —
/// từ chối symlink/non-regular và giới hạn kích thước; trước đây symlink ra file
/// ngoài project đưa thẳng tên dependency vào `mgc.lock`.)
fn collect_manifest_roots(
    lock_path: &Path,
    packages: &[Package],
    root_deps: &mut Vec<String>,
    warnings: &mut Vec<String>,
) {
    let Some(parent) = lock_path.parent() else {
        return;
    };
    let manifest_path = parent.join("package.json");
    if !manifest_path.exists() {
        warnings.push(format!(
            "no package.json beside {}: the imported lock has no root graph, so \
             `mgc why` and freshness cannot tell direct from transitive",
            lock_path
                .file_name()
                .and_then(|name| name.to_str())
                .unwrap_or("lockfile")
        ));
        return;
    }
    let bytes = match crate::parser::read_bounded_regular_file(
        &manifest_path,
        MANIFEST_IMPORT_MAX_BYTES,
        "package.json",
    ) {
        Ok(bytes) => bytes,
        Err(error) => {
            // Loud, not silent: a refused manifest means the root graph is absent,
            // and the user has to know why.
            // (Báo rõ, không im lặng: manifest bị từ chối nghĩa là thiếu root
            // graph và người dùng phải biết lý do.)
            warnings.push(format!(
                "cannot read {} for root detection ({error}); the imported lock will \
                 have no root graph",
                manifest_path.display()
            ));
            return;
        }
    };
    let Ok(json) = serde_json::from_slice::<serde_json::Value>(&bytes) else {
        warnings.push(format!(
            "{} is not valid JSON; the imported lock will have no root graph",
            manifest_path.display()
        ));
        return;
    };
    let mut seen = std::collections::BTreeSet::new();
    for section in ["dependencies", "devDependencies", "optionalDependencies"] {
        let Some(deps) = json.get(section).and_then(|value| value.as_object()) else {
            continue;
        };
        for name in deps.keys() {
            // The lock decides the installed version; a declared package the lock
            // never resolved is reported rather than silently dropped.
            // (Lock quyết định version đã cài; package khai mà lock không resolve
            // được thì báo chứ không bỏ im lặng.)
            let Some(package) = packages.iter().find(|package| package.name == *name) else {
                warnings.push(format!(
                    "'{name}' is declared in {} but absent from the imported lock; no root \
                     recorded for it",
                    manifest_path.display()
                ));
                continue;
            };
            let pin = format_root_pin(
                crate::EcosystemTag::Web,
                &format!("{}@{}", package.name, package.version),
            );
            if seen.insert(pin.clone()) {
                root_deps.push(pin);
            }
        }
    }
}
