//! Go module writer.
//! Writer Go module.

use mgc_types::{Manifest, MgResult};
use std::path::Path;

/// Refuse links/non-regular manifest paths, then publish through a unique
/// same-directory temporary and atomic replace. This prevents a manifest
/// symlink from redirecting a package operation outside the project.
/// Từ chối symlink/file không thường; publish atomic qua temp cùng thư mục.
/// Write go.mod require pins from the manifest for native add.
/// mgc owns the require set. All other directives are preserved verbatim.
/// Versions are always v-prefixed. Non-exact ranges fail closed here;
/// callers must pin through resolve-first before saving.
pub(crate) fn write_go_mod_manifest(root: &Path, manifest: &Manifest) -> MgResult<()> {
    let path = root.join("go.mod");
    let content = mgc_adapter_base::project_file::read_regular_text(&path, "go.mod")?;
    let mut kept: Vec<String> = Vec::new();
    let mut in_require_block = false;
    for line in content.lines() {
        let trimmed = line.trim();
        if in_require_block {
            if trimmed == ")" {
                in_require_block = false;
            }
            continue;
        }
        if trimmed == "require (" {
            in_require_block = true;
            continue;
        }
        if trimmed.starts_with("require ") {
            continue;
        }
        kept.push(line.to_string());
    }
    let mut pins: Vec<(String, String)> = Vec::new();
    for dep in manifest.all_dependencies() {
        let version = dep
            .range
            .satisfying_version()
            .map(|v| v.to_string())
            .unwrap_or_else(|| dep.range.to_string());
        let clean = version.trim_start_matches(['=', 'v', ' ']);
        let parsed = mgc_types::Version::parse(clean).map_err(|_| {
            mgc_types::MgError::Other(format!(
                "go.mod writer needs an exact version for '{}', got '{}' — resolve-first must pin before save",
                dep.name.as_str(),
                dep.range
            ))
        })?;
        pins.push((dep.name.as_str().to_string(), format!("v{parsed}")));
    }
    pins.sort();
    pins.dedup();
    while kept.last().is_some_and(|l| l.trim().is_empty()) {
        kept.pop();
    }
    let mut out = kept.join("\n");
    if !out.is_empty() {
        out.push('\n');
    }
    out.push_str("\nrequire (\n");
    for (name, version) in &pins {
        out.push_str(&format!("\t{name} {version}\n"));
    }
    out.push_str(")\n");
    mgc_adapter_base::project_file::atomic_write_regular(&path, out.as_bytes())?;
    Ok(())
}
