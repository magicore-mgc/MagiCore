//! Tests for deterministic csproj selection.
//! Test chọn csproj xác định.

use super::super::find_csproj;
use std::fs;

#[test]
fn unique_root_csproj_is_selected_case_insensitively() {
    let project = tempfile::tempdir().expect("temporary project");
    let csproj = project.path().join("App.CSPROJ");
    fs::write(&csproj, "<Project/>\n").expect("write root csproj");

    assert_eq!(find_csproj(project.path()), Some(csproj));
}

#[test]
fn multiple_root_csproj_files_are_ambiguous() {
    let project = tempfile::tempdir().expect("temporary project");
    fs::write(project.path().join("One.csproj"), "<Project/>\n").expect("write first project");
    fs::write(project.path().join("Two.csproj"), "<Project/>\n").expect("write second project");

    assert_eq!(find_csproj(project.path()), None);
}

#[test]
fn nested_csproj_is_not_selected() {
    let project = tempfile::tempdir().expect("temporary project");
    let nested = project.path().join("src");
    fs::create_dir_all(&nested).expect("create nested project folder");
    fs::write(nested.join("App.csproj"), "<Project/>\n").expect("write nested csproj");

    assert_eq!(find_csproj(project.path()), None);
}
