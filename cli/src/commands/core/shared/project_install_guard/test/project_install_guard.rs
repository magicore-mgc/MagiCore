//! A second acquisition fails while the first is held.
//! Lần lấy thứ hai fail khi lần đầu còn giữ.

use super::super::acquire;

#[test]
fn a_second_install_on_the_same_project_is_refused() {
    let dir = tempfile::tempdir().unwrap();
    let project = dir.path().join("project");
    std::fs::create_dir_all(&project).unwrap();

    let first = acquire(&project, "lib").expect("the first acquisition succeeds");
    let error = acquire(&project, "lib")
        .err()
        .expect("the second must be refused");
    assert!(
        error
            .to_string()
            .contains("another MagiCore install holds this project"),
        "{error}"
    );
    assert!(
        error.to_string().contains("lib"),
        "the core must be named: {error}"
    );

    drop(first);
    acquire(&project, "lib").expect("released lock can be taken again");
}

#[test]
fn different_projects_do_not_block_each_other() {
    let dir = tempfile::tempdir().unwrap();
    let a = dir.path().join("a");
    let b = dir.path().join("b");
    std::fs::create_dir_all(&a).unwrap();
    std::fs::create_dir_all(&b).unwrap();

    let _a = acquire(&a, "ai").expect("project a");
    acquire(&b, "lib").expect("project b is independent");
}
