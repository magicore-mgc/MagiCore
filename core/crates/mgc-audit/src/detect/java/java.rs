//! Detect the existing Java manifest lane.
//! Nhận diện lane manifest Java hiện có.
use crate::contract::ScanStep;
use std::path::Path;

/// build.gradle / build.gradle.kts / pom.xml → OSV Maven (java).
/// The lane reads gradle/verification-metadata.xml pins, else pom.xml
/// dependencies; without either the scanner reports honest Unsupported
/// with remediation.
/// (build.gradle/pom.xml → OSV Maven.)
pub fn java_step(project_root: &Path) -> Option<ScanStep<'static>> {
    if !project_root.join("build.gradle").is_file()
        && !project_root.join("build.gradle.kts").is_file()
        && !project_root.join("pom.xml").is_file()
    {
        return None;
    }
    let root = project_root.to_path_buf();
    Some(ScanStep {
        ecosystem: "java",
        scanner: "osv-maven",
        run: Box::new(move || {
            let root = root.clone();
            Box::pin(async move { crate::scanners::audit_java(&root).await })
        }),
    })
}
