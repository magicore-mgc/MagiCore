//! .NET manifest writer.
//! Writer manifest .NET.

use super::super::parse_csproj_manifest::parse_csproj_manifest;
use super::super::prune_csproj_references::prune_csproj_references;
use crate::language::find_csproj;
use mgc_types::{Manifest, MgResult};
use std::path::Path;

/// Write csproj PackageReference pins from the manifest (native add).
/// mgc owns the references: each manifest dep becomes (or updates)
/// `<PackageReference Include="id" Version="x" />` inside an ItemGroup
/// (created when absent). Everything else is preserved verbatim.
/// A non-exact range fails closed — callers pin via resolve-first.
/// (Ghi PackageReference csproj từ manifest.)
pub(crate) fn write_csproj_manifest(root: &Path, manifest: &Manifest) -> MgResult<()> {
    // Validate the pre-existing graph before pruning; unresolved references
    // cannot be represented in Manifest and must never be silently deleted.
    // Kiểm tra graph cũ trước khi prune; reference không biểu diễn được không được âm thầm xóa.
    let _current_manifest = parse_csproj_manifest(root)?;
    let path = find_csproj(root).ok_or_else(|| {
        mgc_types::MgError::Other(
            "no .csproj found — scaffold one with `mgc create-lib dotnet`".to_string(),
        )
    })?;
    let content = mgc_adapter_base::project_file::read_regular_text(&path, ".csproj")?;
    let mut pins: Vec<(String, String)> = Vec::new();
    for dep in manifest.all_dependencies() {
        let version = dep
            .range
            .satisfying_version()
            .map(|v| v.to_string())
            .unwrap_or_else(|| dep.range.to_string());
        let clean = version.trim_start_matches(['=', 'v', ' ']);
        mgc_types::Version::parse(clean).map_err(|_| {
            mgc_types::MgError::Other(format!(
                "csproj writer needs an exact version for '{}', got '{}' — resolve-first must pin before save",
                dep.name.as_str(),
                dep.range
            ))
        })?;
        pins.push((dep.name.as_str().to_string(), clean.to_string()));
    }
    pins.sort();
    pins.dedup();
    // Prune stale references first (remove honesty): any PackageReference
    // whose id is NOT in the manifest is deleted — single-line and
    // multi-line forms. A remove that leaves the file behind is a lie.
    // (Xóa reference không còn trong manifest.)
    let keep: std::collections::HashSet<&str> = pins.iter().map(|(n, _)| n.as_str()).collect();
    let mut out = prune_csproj_references(&content, &keep);
    for (name, version) in &pins {
        let escaped = name.replace('&', "&amp;").replace('"', "&quot;");
        // Update in place when the reference already exists (any version).
        let mut replaced = false;
        for line in out.lines() {
            let t = line.trim();
            if t.starts_with("<PackageReference") && t.contains(&format!("Include=\"{escaped}\"")) {
                let indent: String = line.chars().take_while(|c| c.is_whitespace()).collect();
                let replacement = format!(
                    "{indent}<PackageReference Include=\"{escaped}\" Version=\"{version}\" />"
                );
                out = out.replacen(line, &replacement, 1);
                replaced = true;
                break;
            }
        }
        if replaced {
            continue;
        }
        let entry =
            format!("    <PackageReference Include=\"{escaped}\" Version=\"{version}\" />\n");
        if out.contains("</ItemGroup>") {
            out = out.replacen("</ItemGroup>", &format!("{entry}</ItemGroup>"), 1);
        } else if out.contains("</Project>") {
            out = out.replacen(
                "</Project>",
                &format!("  <ItemGroup>\n{entry}  </ItemGroup>\n</Project>"),
                1,
            );
        } else {
            return Err(mgc_types::MgError::Other(
                "csproj has no </Project> root — refusing to write".to_string(),
            ));
        }
    }
    mgc_adapter_base::project_file::atomic_write_regular(&path, out.as_bytes())?;
    Ok(())
}
