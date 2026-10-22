//! Kotlin native audit boundary cases.
//! Các case ranh giới audit native Kotlin.

use crate::audit_tests::tmp;
use mgc_app_adapter::audit::scanner::audit_kotlin;
use mgc_types::adapter::ScannerStatus;

#[tokio::test]
async fn audit_kotlin_without_native_lock_evidence_is_unsupported() {
    let dir = tmp("kotlin-no-tool");
    std::fs::write(dir.join("build.gradle"), "// empty\n").unwrap();

    let report = audit_kotlin(&dir).await.unwrap();
    assert_eq!(report.vulnerability_count, 0);
    assert!(matches!(
        report.scanner_status,
        ScannerStatus::UnsupportedEcosystem { .. }
    ));
}

#[tokio::test]
async fn audit_kotlin_does_not_delegate_for_unresolved_project() {
    let dir = tmp("kotlin-no-delegation");
    // The Gradle fixture defines a sentinel task that would write this report
    // if invoked; this audit call should leave the report absent. This does
    // not claim that Gradle ran or that a successful dependency scan occurred.
    // Fixture Gradle định nghĩa task sentinel ghi report nếu bị gọi; audit
    // phải để report vắng mặt. Case này không chứng minh Gradle đã chạy hay
    // một lượt quét dependency thành công.
    std::fs::write(
        dir.join("settings.gradle"),
        "rootProject.name = 'kotlin-success-fixture'\n",
    )
    .unwrap();
    std::fs::write(
        dir.join("build.gradle"),
        r#"
import groovy.json.JsonOutput

tasks.register('dependencyCheckAnalyze') {
    doLast {
        def reportDir = new File(projectDir, 'build/reports')
        reportDir.mkdirs()
        def report = [
          projectInfo: [name: 'kotlin-success-fixture'],
          dependencies: [
            [
              fileName: 'jackson-databind-2.9.10.8.jar',
              packages: [
                [package: [id: 'pkg:maven/com.fasterxml.jackson.core/jackson-databind@2.9.10.8']]
              ],
              vulnerabilities: [
                [name: 'CVE-2020-25649', severity: 'HIGH', description: 'XXE in jackson-databind',
                 references: [[url: 'https://nvd.nist.gov/vuln/detail/CVE-2020-25649']]]
              ]
            ],
            [
              fileName: 'okhttp-4.12.0.jar',
              packages: [
                [package: [id: 'pkg:maven/com.squareup.okhttp3/okhttp@4.12.0']]
              ],
              vulnerabilities: []
            ]
          ]
        ]
        new File(reportDir, 'dependency-check-report.json').text = JsonOutput.toJson(report)
    }
}

"#,
    )
    .unwrap();

    let report = audit_kotlin(&dir).await.unwrap();
    assert!(matches!(
        report.scanner_status,
        ScannerStatus::UnsupportedEcosystem { .. }
    ));
    assert!(
        !dir.join("build/reports/dependency-check-report.json")
            .exists()
    );
}
