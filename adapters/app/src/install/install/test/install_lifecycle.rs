//! App install lifecycle regression tests.
//! Test hồi quy lifecycle install App.

use crate::install::run_install;
use mgc_types::ResolvedGraph;
use mgc_types::adapter::InstallOptions;

#[tokio::test]
async fn native_app_offline_install_fails_before_project_mutation() {
    let project = tempfile::tempdir().unwrap();
    std::fs::write(project.path().join("pubspec.yaml"), "name: sample\n").unwrap();

    let error = run_install(
        crate::language::AppLanguage::Swift,
        &ResolvedGraph::empty(),
        project.path(),
        InstallOptions {
            offline: true,
            ..InstallOptions::default()
        },
        None,
        Vec::new(),
    )
    .await
    .expect_err("App offline reinstall is not implemented yet");

    assert!(error.to_string().contains("offline install is unsupported"));
    assert!(
        !project.path().join("mgc.lock").exists(),
        "unsupported offline install must not publish a lock"
    );
    assert!(
        !project.path().join(".dart_tool").exists(),
        "unsupported offline install must not materialize Flutter config"
    );
}
