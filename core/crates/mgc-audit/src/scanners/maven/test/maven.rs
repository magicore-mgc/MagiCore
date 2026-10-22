//! Maven audit coordinator tests.
//! Test bộ điều phối audit Maven.

use super::*;
use mgc_types::adapter::ScannerStatus;

#[test]
fn gradle_verification_precedes_pom_without_network() {
    let directory = tempfile::tempdir().expect("temporary Maven project");
    std::fs::create_dir_all(directory.path().join("gradle"))
        .expect("create Gradle metadata folder");
    std::fs::write(
        directory.path().join("gradle/verification-metadata.xml"),
        "<components>\n<dependency group=\"example\" name=\"missing-version\">\n</components>\n",
    )
    .expect("write incomplete Gradle verification metadata");
    std::fs::write(
        directory.path().join("pom.xml"),
        r#"<project>
<dependencies>
<dependency>
<groupId>example</groupId>
<artifactId>pom-only</artifactId>
</dependency>
</dependencies>
</project>"#,
    )
    .expect("write valid POM beside Gradle metadata");

    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("build local runtime");
    let report = runtime
        .block_on(audit_java(directory.path()))
        .expect("audit project");
    assert_eq!(report.packages_audited, 0);
    match &report.scanner_status {
        ScannerStatus::Partial { reasons, .. } => assert!(reasons.iter().any(|reason| {
            reason.contains("gradle dependency line missing group/name/version")
        })),
        other => panic!("Gradle metadata must retain precedence, got {other:?}"),
    }
}
