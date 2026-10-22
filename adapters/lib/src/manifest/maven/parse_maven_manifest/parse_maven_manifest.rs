//! Maven manifest parser.
//! Parser manifest Maven.

use mgc_types::{DependencySpec, Ecosystem, Manifest, MgResult, PackageName, VersionRange};
use std::path::Path;

/// Parse a project pom.xml into a Manifest (read-only — mgc never rewrites
/// pom.xml). Only the SAME compile/runtime deps the native engine uses enter
/// the manifest, keyed as `groupId:artifactId`; local properties and literal
/// dependencyManagement versions are resolved, while unresolved versions
/// fail closed rather than silently disappearing. Gradle projects get an honest empty manifest — build
/// scripts are not parseable without the toolchain and resolve fails closed
/// downstream with guidance.
/// Đọc pom.xml của project thành Manifest (chỉ đọc — mgc không bao giờ viết
/// lại pom.xml). Chỉ những dep compile/runtime mà engine native dùng vào
/// manifest, khóa theo `groupId:artifactId`; property cục bộ và version
/// literal trong dependencyManagement được giải; version chưa giải được
/// sẽ fail-closed thay vì bị âm thầm bỏ qua.
/// Project gradle nhận manifest rỗng trung thực — build script không parse
/// được nếu không có toolchain và resolve sẽ fail-closed kèm hướng dẫn.
pub(crate) fn parse_maven_manifest(root: &Path) -> MgResult<Manifest> {
    let pom_path = root.join("pom.xml");
    match std::fs::symlink_metadata(&pom_path) {
        Ok(metadata) if metadata.file_type().is_file() => {}
        Ok(metadata) if metadata.file_type().is_symlink() => {
            return Err(mgc_types::MgError::Other(
                "refusing symlinked Maven manifest pom.xml".into(),
            ));
        }
        Ok(_) => {
            return Err(mgc_types::MgError::Other(
                "Maven manifest pom.xml is not a regular file".into(),
            ));
        }
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
            return Ok(Manifest::new("java-gradle-lib", Ecosystem::Lib));
        }
        Err(error) => {
            return Err(mgc_types::MgError::Other(format!(
                "inspect pom.xml: {error}"
            )));
        }
    }
    let content = mgc_adapter_base::project_file::read_regular_text(&pom_path, "pom.xml")?;
    let (group, artifact) = mgc_resolver::protocols::maven::pom_project_coordinates(&content)?
        .unwrap_or_else(|| ("unknown".to_string(), "unknown".to_string()));
    let mut manifest = Manifest::new(&format!("{group}:{artifact}"), Ecosystem::Lib);
    let (deps, _) = mgc_resolver::protocols::maven::collect_pom_dependencies(&content)?;
    let properties = mgc_resolver::protocols::maven::collect_pom_properties(&content)?;
    let managed = mgc_resolver::protocols::maven::collect_managed_versions(&content)?;
    for dep in deps {
        let scope = dep.scope.as_deref().unwrap_or("compile");
        if !matches!(scope, "compile" | "runtime" | "") || dep.optional {
            continue;
        }
        let (Some(g), Some(a)) = (&dep.group, &dep.artifact) else {
            return Err(mgc_types::MgError::Other(
                "pom.xml contains a compile/runtime dependency without groupId or artifactId; refusing to mutate an incomplete manifest".into(),
            ));
        };
        let dep_name = PackageName::new(format!("{g}:{a}")).map_err(|e| {
            mgc_types::MgError::Other(format!(
                "pom.xml contains invalid dependency coordinates '{g}:{a}': {e}"
            ))
        })?;
        let raw_version = dep
            .version
            .as_deref()
            .or_else(|| managed.get(&(g.clone(), a.clone())).map(String::as_str))
            .ok_or_else(|| {
                mgc_types::MgError::Other(format!(
                    "pom.xml dependency '{}' cannot be resolved from this pom.xml; parent/BOM inheritance is not resolved here, refusing to mutate an incomplete manifest",
                    dep_name.as_str()
                ))
            })?;
        let version = mgc_resolver::protocols::maven::substitute_properties(raw_version, &properties)
            .ok_or_else(|| {
                mgc_types::MgError::Other(format!(
                    "pom.xml dependency '{}' uses an unresolved version property '{}'; refusing to mutate an incomplete manifest",
                    dep_name.as_str(), raw_version
                ))
            })?;
        let range = VersionRange::parse(&version).map_err(|e| {
            mgc_types::MgError::Other(format!(
                "pom.xml dependency '{}' has unsupported version '{}': {e}",
                dep_name.as_str(),
                version
            ))
        })?;
        let spec = DependencySpec::new(dep_name, range);
        manifest.add_dep(spec, false, false, false);
    }
    Ok(manifest)
}
