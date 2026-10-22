//! .NET manifest parser.
//! Parser manifest .NET.

use super::super::has_unmodeled_package_reference_semantics::has_unmodeled_package_reference_semantics;
use mgc_types::{DependencySpec, Ecosystem, Manifest, MgResult, PackageName, VersionRange};
use std::path::Path;

/// Parse the project's .csproj `<PackageReference>` entries into a Manifest.
/// Unresolved versions and conditional/update/remove semantics fail closed;
/// they cannot be represented by the flat dependency model.
/// Đọc `<PackageReference>` của .csproj thành Manifest (chỉ đọc — mgc
/// từ chối version chưa giải và semantics condition/update/remove mà graph
/// phẳng chưa biểu diễn được.
pub(crate) fn parse_csproj_manifest(root: &Path) -> MgResult<Manifest> {
    use crate::language::find_csproj;
    let Some(csproj) = find_csproj(root) else {
        return Err(mgc_types::MgError::Other(
            "no unique regular root-level .csproj was found; refusing to return an empty .NET dependency manifest".into(),
        ));
    };
    let content = mgc_adapter_base::project_file::read_regular_text(&csproj, ".csproj")?;
    if has_unmodeled_package_reference_semantics(&content) {
        return Err(mgc_types::MgError::Unsupported {
            core: "lib",
            capability: ".NET conditional/Update/Remove PackageReference",
            guidance: "MagiCore does not yet preserve conditional, Update, or Remove PackageReference semantics; these package graphs are refused before mutation".to_string(),
        });
    }
    let name = csproj
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("dotnet-lib")
        .to_string();
    let (refs, unresolved) = mgc_resolver::protocols::nuget::parse_package_references(&content);
    if !unresolved.is_empty() {
        return Err(mgc_types::MgError::Unsupported {
            core: "lib",
            capability: ".NET PackageReference with unresolved version",
            guidance: format!(
                "MagiCore cannot safely mutate this project because these PackageReference entries have floating or missing versions: {}",
                unresolved.join(", ")
            ),
        });
    }
    let mut manifest = Manifest::new(&name, Ecosystem::Lib);
    for r in refs {
        let Some(version) = &r.version else {
            continue;
        };
        let dep_name = PackageName::new(r.id.clone()).map_err(|error| {
            mgc_types::MgError::Other(format!(
                "invalid NuGet PackageReference id '{}': {error}",
                r.id
            ))
        })?;
        let range = VersionRange::parse(version).map_err(|error| {
            mgc_types::MgError::Unsupported {
                core: "lib",
                capability: ".NET PackageReference version range",
                guidance: format!(
                    "MagiCore cannot safely mutate PackageReference '{}' with version '{version}': {error}",
                    r.id
                ),
            }
        })?;
        let spec = DependencySpec::new(dep_name, range);
        manifest.add_dep(spec, false, false, false);
    }
    Ok(manifest)
}
