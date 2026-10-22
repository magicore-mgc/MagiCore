//! Import legacy lockfiles — điều phối nhập lockfile cũ.

use crate::schema::{Provenance, SOURCE_KIND_REGISTRY_IMPORT};
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
        NPM_LOCKFILE => parse_npm(&content, &mut warnings)?,
        PNPM_LOCKFILE => parse_pnpm(&content, &mut warnings)?,
        YARN_LOCKFILE => parse_yarn(&content)?,
        BUN_LOCKFILE => parse_bun(&content, &mut skipped)?,
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
