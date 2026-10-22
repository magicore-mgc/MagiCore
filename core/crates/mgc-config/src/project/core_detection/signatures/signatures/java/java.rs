//! java manifest recognition.
//! Nhận diện manifest java.
use super::super::shared::{signature_file_exists, text_contains_any};
use crate::project::read_regular_project_text;
use std::path::Path;
pub(in crate::project::project::core_detection) fn detect_jvm_cores(
    project_root: &Path,
) -> anyhow::Result<Vec<String>> {
    let mut java_manifests = Vec::new();
    for name in ["pom.xml", "build.gradle", "build.gradle.kts"] {
        if signature_file_exists(&project_root.join(name))? {
            java_manifests.push(name);
        }
    }
    let mut java_cores = std::collections::BTreeSet::new();
    for name in java_manifests {
        let content = read_regular_project_text(&project_root.join(name), "JVM manifest")?
            .ok_or_else(|| anyhow::anyhow!("JVM manifest disappeared during core detection"))?;
        if text_contains_any(
            &content,
            &[
                "spring-boot-starter-web",
                "spring-boot-starter-webflux",
                "io.quarkus",
                "quarkus-rest",
            ],
        ) {
            java_cores.insert("web");
        } else if text_contains_any(
            &content,
            &["com.android.application", "com.android.tools.build:gradle"],
        ) {
            java_cores.insert("app");
        } else {
            java_cores.insert("lib");
        }
    }
    Ok(java_cores.into_iter().map(str::to_string).collect())
}
