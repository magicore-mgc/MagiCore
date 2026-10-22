//! Fail-closed patch preflight unit test.
//! Test preflight từ chối patch fail-closed.

use mgc_types::adapter::{InstallOptions, ResolvedGraph};

fn tempdir_real() -> std::io::Result<tempfile::TempDir> {
    let canonical = std::env::temp_dir().canonicalize()?;
    tempfile::tempdir_in(canonical)
}

#[tokio::test]
async fn install_fails_closed_when_project_has_unapplied_patch_specs() {
    let dir = tempdir_real().unwrap();
    let mut config = mgc_config::project::ProjectConfig::new("patched-web", "web");
    config.patches.push(mgc_types::PatchSpec::new(
        "example".to_string(),
        mgc_types::VersionRange::star(),
        "example.patch".to_string(),
        "sha256-deadbeef".to_string(),
    ));
    config.save(dir.path()).unwrap();

    let result = crate::install::run_install(
        "https://registry.example",
        None,
        None,
        None,
        None,
        &ResolvedGraph::empty(),
        dir.path(),
        InstallOptions::default(),
    )
    .await;

    let error = result.expect_err("configured patches must never be silently ignored");
    assert!(error.to_string().contains("patch"));
    assert!(
        !dir.path().join("node_modules").exists(),
        "install must reject before creating or mutating the dependency tree"
    );
}
