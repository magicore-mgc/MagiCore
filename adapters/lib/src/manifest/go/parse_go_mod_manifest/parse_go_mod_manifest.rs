//! Go module parser.
//! Parser Go module.

use mgc_types::{DependencySpec, Ecosystem, Manifest, MgResult, PackageName, VersionRange};
use std::path::Path;

/// Parse go.mod into a Manifest for MGC-owned resolve/add/remove/update.
/// MGC owns the require set it writes; unrelated module directives are
/// preserved. Dependencies keep
/// their FULL module path (PackageName's repo-path form) so the native
/// GoModProtocol can resolve them; the project's own `replace` directives
/// rewrite pins and `exclude` drops exact (module, version) pairs — the
/// effective module graph, honestly.
/// Đọc go.mod thành Manifest cho resolve/add/remove/update do MGC sở hữu.
/// MGC sở hữu require set được ghi; directive module không liên quan được giữ.
/// Dependency giữ
/// NGUYÊN path module (dạng repo-path của PackageName) để GoModProtocol
/// native resolve được; directive `replace` của project viết lại pin và
/// `exclude` loại cặp (module, version) đúng đó — graph module hiệu lực,
/// trung thực.
pub(crate) fn parse_go_mod_manifest(root: &Path) -> MgResult<Manifest> {
    let content =
        mgc_adapter_base::project_file::read_regular_text(&root.join("go.mod"), "go.mod")?;
    let mut name = "unknown".to_string();
    for line in content.lines() {
        let trimmed = line.trim();
        if let Some(module) = trimmed.strip_prefix("module ") {
            name = module.trim().to_string();
            break;
        }
    }
    // One shared parser with the native engine (require/replace/exclude,
    // single-line and block forms).
    // (Một parser dùng chung với engine native — require/replace/exclude,
    // dạng một dòng và khối.)
    let gomod = mgc_resolver::protocols::go::parse_go_mod(&content)?;

    let mut manifest = Manifest::new(&name, Ecosystem::Lib);
    for req in &gomod.requires {
        // `exclude` removes that exact (module, version) pin from the graph.
        // (`exclude` loại pin (module, version) đúng đó khỏi graph.)
        if gomod
            .excludes
            .iter()
            .any(|(p, v)| p == &req.path && v == &req.version)
        {
            continue;
        }
        // `replace` rewrites the pin — version-scoped when the directive
        // pins the old version.
        // (`replace` viết lại pin — theo version khi directive ghim version
        // cũ.)
        let mut path = req.path.clone();
        let mut version = req.version.clone();
        for rep in &gomod.replaces {
            let version_ok = rep.old_version.as_ref().is_none_or(|v| *v == req.version);
            if rep.old_path == path && version_ok {
                path = rep.new_path.clone();
                if let Some(nv) = &rep.new_version {
                    version = nv.clone();
                }
                break;
            }
        }
        if let Ok(dep_name) = PackageName::new(path) {
            let Ok(range) = VersionRange::parse(&version) else {
                continue;
            };
            let spec = DependencySpec::new(dep_name, range);
            manifest.add_dep(spec, false, false, false);
        }
    }
    Ok(manifest)
}
