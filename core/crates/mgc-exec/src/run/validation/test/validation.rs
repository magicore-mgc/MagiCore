// Tests for project-bound argument validation.
// Test xác thực đối số theo ranh giới dự án.

use super::{ArgumentValidationError, validate_args_no_traversal};
use std::fs;

#[test]
fn accepts_dotted_values_without_a_project_root() {
    let args = [
        "1.0..2".to_owned(),
        "config..backup".to_owned(),
        "--constraint=1.0..2".to_owned(),
    ];

    validate_args_no_traversal(&args, None).expect("non-path dotted values should pass");
}

#[test]
fn rejects_parent_components_without_a_project_root() {
    let args = ["modules/../outside".to_owned()];

    let error = validate_args_no_traversal(&args, None)
        .expect_err("parent components cannot be checked without a project root");
    assert!(matches!(
        error.downcast_ref::<ArgumentValidationError>(),
        Some(ArgumentValidationError::MissingProjectRoot { .. })
    ));
}

#[cfg(unix)]
#[test]
fn rejects_absolute_unix_paths_without_a_project_root() {
    let args = ["/etc/passwd".to_owned()];

    let error = validate_args_no_traversal(&args, None)
        .expect_err("absolute paths cannot be checked without a project root");
    assert!(matches!(
        error.downcast_ref::<ArgumentValidationError>(),
        Some(ArgumentValidationError::MissingProjectRoot { .. })
    ));
}

#[cfg(windows)]
#[test]
fn rejects_drive_paths_without_a_project_root() {
    let args = [r"C:\Windows\System32".to_owned()];

    let error = validate_args_no_traversal(&args, None)
        .expect_err("drive paths cannot be checked without a project root");
    assert!(matches!(
        error.downcast_ref::<ArgumentValidationError>(),
        Some(ArgumentValidationError::MissingProjectRoot { .. })
    ));
}

#[test]
fn accepts_existing_files_inside_the_project() {
    let project = tempfile::tempdir().expect("create project directory");
    fs::create_dir(project.path().join("src")).expect("create source directory");
    fs::write(project.path().join("src/main.rs"), "fn main() {}").expect("create source file");
    let args = ["src/main.rs".to_owned()];

    validate_args_no_traversal(&args, Some(project.path()))
        .expect("project-local paths should pass");
}

#[test]
fn rejects_nonexistent_paths_that_escape_the_project() {
    let project = tempfile::tempdir().expect("create project directory");
    let args = ["../missing/outside.txt".to_owned()];

    let error = validate_args_no_traversal(&args, Some(project.path()))
        .expect_err("nonexistent paths escaping the project must be rejected");
    assert!(matches!(
        error.downcast_ref::<ArgumentValidationError>(),
        Some(ArgumentValidationError::PathOutsideProject { .. })
    ));
}

#[cfg(unix)]
#[test]
fn rejects_missing_files_below_a_symlink_escape() {
    use std::os::unix::fs::symlink;

    let workspace = tempfile::tempdir().expect("create workspace directory");
    let project = workspace.path().join("project");
    let outside = workspace.path().join("outside");
    fs::create_dir(&project).expect("create project directory");
    fs::create_dir(&outside).expect("create outside directory");
    symlink(&outside, project.join("external")).expect("create outside symlink");
    let args = ["external/new-file.txt".to_owned()];

    let error = validate_args_no_traversal(&args, Some(&project))
        .expect_err("a missing file below an outside symlink must be rejected");
    assert!(matches!(
        error.downcast_ref::<ArgumentValidationError>(),
        Some(ArgumentValidationError::PathOutsideProject { .. })
    ));
}

#[cfg(unix)]
#[test]
fn fails_closed_for_a_dangling_symlink_in_a_missing_path() {
    use std::os::unix::fs::symlink;

    let workspace = tempfile::tempdir().expect("create workspace directory");
    let project = workspace.path().join("project");
    fs::create_dir(&project).expect("create project directory");
    symlink(
        workspace.path().join("not-created"),
        project.join("dangling"),
    )
    .expect("create dangling symlink");
    let args = ["dangling/new-file.txt".to_owned()];

    let error = validate_args_no_traversal(&args, Some(&project))
        .expect_err("a dangling symlink cannot be safely resolved");
    assert!(matches!(
        error.downcast_ref::<ArgumentValidationError>(),
        Some(ArgumentValidationError::CannotResolvePath { .. })
    ));
}
