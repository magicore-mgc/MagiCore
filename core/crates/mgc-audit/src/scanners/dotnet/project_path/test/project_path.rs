//! Verify safe project path lookup on case-sensitive and case-insensitive hosts.
//! Kiểm tra lookup project path an toàn trên host phân biệt hoặc không phân biệt hoa thường.

#[test]
fn resolve_project_path_tolerates_case_drift() {
    // Windows-authored solution paths must resolve on case-sensitive hosts.
    // Path từ solution Windows cần resolve được trên host phân biệt hoa thường.
    let dir = tempfile::tempdir().expect("tempdir");
    std::fs::create_dir_all(dir.path().join("a")).unwrap();
    std::fs::write(dir.path().join("a").join("a.csproj"), "<Project/>\n").unwrap();
    let hit = super::resolve_project_path(dir.path(), "a/A.csproj").expect("case drift resolves");
    assert!(hit.is_file());
    assert!(super::resolve_project_path(dir.path(), "a/missing.csproj").is_none());
    assert!(super::resolve_project_path(dir.path(), "../escape.csproj").is_none());
}

#[cfg(target_os = "linux")]
#[test]
fn resolve_project_path_refuses_ambiguous_case_fold_matches() {
    let directory = tempfile::tempdir().expect("tempdir");
    std::fs::write(directory.path().join("A.csproj"), "<Project/>\n").unwrap();
    std::fs::write(directory.path().join("a.csproj"), "<Project/>\n").unwrap();

    assert!(super::resolve_project_path(directory.path(), "a.CsProj").is_none());
}
