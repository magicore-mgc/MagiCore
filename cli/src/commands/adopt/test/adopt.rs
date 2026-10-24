//! Adoption refuses anything the directory does not prove.
//! Adopt từ chối mọi thứ thư mục không chứng minh.

use super::super::run;

#[test]
fn a_matching_marker_is_adopted() {
    let dir = tempfile::tempdir().unwrap();
    let marker = dir.path().join(".github").join("workflows");
    std::fs::create_dir_all(&marker).unwrap();
    assert!(
        marker.exists(),
        "marker directory must exist at {}",
        marker.display()
    );
    run("cicd", dir.path()).expect("a workflows directory proves a cicd project");
    assert!(dir.path().join(".mgc.core").exists());
    assert!(dir.path().join("mgc.toml").exists());
}

#[test]
fn a_directory_without_the_marker_is_refused() {
    let dir = tempfile::tempdir().unwrap();
    let error = run("cicd", dir.path()).expect_err("no marker means no adoption");
    assert!(
        error
            .to_string()
            .contains("does not look like a cicd project"),
        "{error}"
    );
    assert!(!dir.path().join(".mgc.core").exists());
    assert!(!dir.path().join("mgc.toml").exists());
}

#[test]
fn an_existing_project_is_not_overwritten() {
    let dir = tempfile::tempdir().unwrap();
    std::fs::create_dir_all(dir.path().join("cdk.json").parent().unwrap()).unwrap();
    std::fs::write(dir.path().join("cdk.json"), "{}").unwrap();
    std::fs::write(dir.path().join("mgc.toml"), "[project]\nname = \"kept\"\n").unwrap();
    let error = run("clo", dir.path()).expect_err("adoption must not replace identity");
    assert!(
        error.to_string().contains("already a MagiCore project"),
        "{error}"
    );
    let kept = std::fs::read_to_string(dir.path().join("mgc.toml")).unwrap();
    assert!(
        kept.contains("kept"),
        "the original manifest must survive: {kept}"
    );
}

#[test]
fn a_core_with_an_initializer_points_at_its_create_command() {
    let dir = tempfile::tempdir().unwrap();
    let error = run("web", dir.path()).expect_err("web has create-web");
    assert!(error.to_string().contains("mgc create-web"), "{error}");
}
