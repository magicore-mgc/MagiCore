//! Project-config identity lookup contract.

use super::super::read;

#[test]
fn reads_core_from_project_config_when_marker_is_absent() {
    let project = tempfile::tempdir().unwrap();
    std::fs::write(
        project.path().join("mgc.toml"),
        "name = \"fixture\"\necosystem = \"game\"\n",
    )
    .unwrap();

    assert_eq!(read(project.path()).unwrap().as_deref(), Some("game"));
}
