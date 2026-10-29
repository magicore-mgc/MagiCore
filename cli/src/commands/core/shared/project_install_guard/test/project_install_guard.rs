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

/// SEC-08: a repository that ships `.magicore` as a symlink is refused before
/// any store write, so a planted link cannot redirect project metadata.
/// (SEC-08: repo ship `.magicore` là symlink bị từ chối trước mọi ghi store,
/// nên link bị cài không chuyển hướng được metadata project.)
#[cfg(unix)]
#[test]
fn symlinked_project_metadata_directory_is_refused() {
    let dir = tempfile::tempdir().unwrap();
    let project = dir.path().join("project");
    let elsewhere = dir.path().join("elsewhere");
    std::fs::create_dir_all(&project).unwrap();
    std::fs::create_dir_all(&elsewhere).unwrap();
    std::os::unix::fs::symlink(&elsewhere, project.join(".magicore")).unwrap();

    let error = acquire(&project, "web")
        .err()
        .expect("a symlinked .magicore must be refused");
    assert!(
        error.to_string().contains("is a symlink"),
        "the reason must name the link: {error}"
    );
    assert!(
        std::fs::read_dir(&elsewhere).unwrap().next().is_none(),
        "nothing may be written through the planted link"
    );
}

/// A plain `.magicore` file is not a directory and is refused the same way.
/// (`.magicore` là file thường không phải thư mục và bị từ chối tương tự.)
#[test]
fn project_metadata_file_is_refused() {
    let dir = tempfile::tempdir().unwrap();
    let project = dir.path().join("project");
    std::fs::create_dir_all(&project).unwrap();
    std::fs::write(project.join(".magicore"), b"not a directory").unwrap();

    let error = acquire(&project, "ai")
        .err()
        .expect("a .magicore file must be refused");
    assert!(error.to_string().contains("is not a directory"), "{error}");
}
