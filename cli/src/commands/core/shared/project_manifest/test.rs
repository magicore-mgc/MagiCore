//! Manifest-probe tests — test probe manifest.
//! The probe decides which languages a command recognises, so each edge that
//! once produced a wrong answer is pinned here.
//! (Probe quyết định lệnh nhận diện ngôn ngữ nào, nên mọi biên từng trả lời sai
//! đều được ghim.)

use super::implementation::{declares_dotnet_project, has_file_with_extension};

#[test]
fn a_named_project_file_is_found() {
    let dir = tempfile::tempdir().expect("temp dir");
    std::fs::write(dir.path().join("App.csproj"), "<Project/>").expect("write");
    assert!(has_file_with_extension(dir.path(), "csproj").expect("probe"));
    assert!(declares_dotnet_project(dir.path()));
}

#[test]
fn a_solution_file_counts_as_dotnet() {
    let dir = tempfile::tempdir().expect("temp dir");
    std::fs::write(dir.path().join("All.sln"), "").expect("write");
    assert!(declares_dotnet_project(dir.path()));
}

#[test]
fn the_extension_match_ignores_case() {
    let dir = tempfile::tempdir().expect("temp dir");
    std::fs::write(dir.path().join("App.CSPROJ"), "").expect("write");
    assert!(has_file_with_extension(dir.path(), "csproj").expect("probe"));
}

#[test]
fn a_directory_with_the_extension_is_not_a_project_file() {
    let dir = tempfile::tempdir().expect("temp dir");
    std::fs::create_dir(dir.path().join("Trap.csproj")).expect("mkdir");
    assert!(!has_file_with_extension(dir.path(), "csproj").expect("probe"));
}

#[test]
fn a_missing_root_is_not_an_error() {
    assert!(
        !has_file_with_extension(
            std::path::Path::new("/nonexistent/mgc-probe-root"),
            "csproj"
        )
        .expect("a missing root answers false")
    );
}

#[test]
fn an_empty_directory_declares_nothing() {
    let dir = tempfile::tempdir().expect("temp dir");
    assert!(!declares_dotnet_project(dir.path()));
}
