use super::super::pom_xml::{element_blocks, normalize_pom_xml, remove_element_blocks};
use super::super::types::PomDependency;
use mgc_types::{MgError, MgResult};

/// Collect a POM's real dependencies: `dependencyManagement` blocks are
/// removed first; returns `(deps, packaging)`. Public so adapters parse
/// their project pom.xml with the SAME parser as the engine.
/// Gom dependency thật của POM: khối `dependencyManagement` bị cắt trước;
/// trả `(deps, packaging)`. Public để adapter parse pom.xml của project
/// bằng CÙNG parser với engine.
pub fn collect_pom_dependencies(pom: &str) -> MgResult<(Vec<PomDependency>, Option<String>)> {
    let normalized = normalize_pom_xml(pom)?;
    let pom = normalized.as_str();
    let packaging = element_blocks(pom, "packaging")
        .first()
        .map(|s| s.trim().to_string());
    let stripped = remove_element_blocks(pom, "dependencyManagement");
    // Project graph = TOP-LEVEL <dependencies> only. Plugin <dependencies>
    // (surefire/javadoc/site plugin deps), <reporting> and <profiles> are
    // NEVER on the compile/runtime classpath — collecting them pulled
    // plugin-only artifacts (e.g. surefire-junit47's lineage) into the
    // graph and failed versionless downstream (doxia-module-xhtml).
    // (Chỉ <dependencies> cấp cao nhất — dep của plugin không vào graph.)
    let stripped = remove_element_blocks(&stripped, "build");
    let stripped = remove_element_blocks(&stripped, "reporting");
    let stripped = remove_element_blocks(&stripped, "profiles");
    let mut deps = Vec::new();
    for block in element_blocks(&stripped, "dependencies") {
        for dep_xml in element_blocks(block, "dependency") {
            let field = |tag: &str| {
                element_blocks(dep_xml, tag)
                    .first()
                    .map(|s| s.trim().to_string())
            };
            let group = field("groupId");
            let artifact = field("artifactId");
            match (group.as_deref(), artifact.as_deref()) {
                (Some(group), Some(artifact)) if !group.is_empty() && !artifact.is_empty() => {}
                (None | Some(""), None | Some("")) => {
                    return Err(MgError::Other(
                        "Maven dependency is missing required groupId and artifactId; refusing to silently omit it from the resolved graph".to_string(),
                    ));
                }
                (None | Some(""), _) => {
                    return Err(MgError::Other(
                        "Maven dependency is missing required groupId; refusing to silently omit it from the resolved graph".to_string(),
                    ));
                }
                (_, None | Some("")) => {
                    return Err(MgError::Other(
                        "Maven dependency is missing required artifactId; refusing to silently omit it from the resolved graph".to_string(),
                    ));
                }
                _ => {}
            }
            deps.push(PomDependency {
                group,
                artifact,
                version: field("version"),
                scope: field("scope"),
                optional: field("optional").is_some_and(|v| v.eq_ignore_ascii_case("true")),
            });
        }
    }
    Ok((deps, packaging))
}
