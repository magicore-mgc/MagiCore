//! Tests the append docker readme behavior.
//! Kiểm tra hành vi của owner append_docker_readme.

use super::super::append_docker_readme::append_docker_readme;
#[test]
fn docker_readme_append_is_idempotent() {
    let project = tempfile::tempdir().unwrap();
    std::fs::write(project.path().join("README.md"), "# Fixture\n").unwrap();

    append_docker_readme(project.path()).unwrap();
    let once = std::fs::read_to_string(project.path().join("README.md")).unwrap();
    append_docker_readme(project.path()).unwrap();
    let twice = std::fs::read_to_string(project.path().join("README.md")).unwrap();

    assert_eq!(once, twice);
    assert_eq!(twice.matches("## Docker").count(), 1);
}
