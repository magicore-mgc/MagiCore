//! Verify root and solution locks aggregate with one shared identity set.
//! Kiểm tra lock root và solution dùng chung một tập identity khi gom.

#[test]
fn solution_and_root_locks_are_collected_and_deduplicated() {
    let project = tempfile::tempdir().expect("tempdir");
    std::fs::create_dir_all(project.path().join("src")).unwrap();
    std::fs::write(project.path().join("src/Api.csproj"), "<Project/>\n").unwrap();
    std::fs::write(
        project.path().join("App.sln"),
        r#"Project("{A}") = "Api", "src\Api.csproj", "{B}""#,
    )
    .unwrap();
    std::fs::write(
        project.path().join("src/packages.lock.json"),
        r#"{"dependencies":{"net8.0":{"Shared":{"resolved":"1.0"},"SolutionOnly":{"resolved":"2.0"}}}}"#,
    )
    .unwrap();
    std::fs::write(
        project.path().join("packages.lock.json"),
        r#"{"dependencies":{"net8.0":{"Shared":{"resolved":"1.0"},"RootOnly":{"resolved":"3.0"}}}}"#,
    )
    .unwrap();

    let (pins, skipped, recognized) = super::collect_dotnet_pins(project.path()).unwrap();

    assert!(recognized);
    assert!(skipped.is_empty());
    assert_eq!(
        pins.iter().map(|pin| pin.name.as_str()).collect::<Vec<_>>(),
        vec!["Shared", "SolutionOnly", "RootOnly"]
    );
}

#[cfg(unix)]
#[test]
fn solution_symlink_project_outside_root_is_skipped() {
    let project = tempfile::tempdir().expect("project tempdir");
    let outside = tempfile::tempdir().expect("outside tempdir");
    std::fs::write(outside.path().join("Api.csproj"), "<Project/>\n").unwrap();
    std::os::unix::fs::symlink(
        outside.path().join("Api.csproj"),
        project.path().join("Api.csproj"),
    )
    .unwrap();
    std::fs::write(
        project.path().join("App.sln"),
        r#"Project("{A}") = "Api", "Api.csproj", "{B}""#,
    )
    .unwrap();

    let (pins, skipped, recognized) = super::collect_dotnet_pins(project.path()).unwrap();

    assert!(recognized);
    assert!(pins.is_empty());
    assert!(skipped.iter().any(|reason| reason.contains("refused")));
}
