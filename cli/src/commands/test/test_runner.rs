// Test-runner selection contracts — khóa lựa chọn runner theo ownership.
use super::{detect_test_core, detect_test_runner, resolve_mgc_toml_script};

#[test]
fn flutter_test_disables_implicit_pub_resolution() {
    let dir = tempfile::tempdir().unwrap();
    std::fs::write(dir.path().join("pubspec.yaml"), "name: fixture\n").unwrap();

    let (runner, args) = detect_test_runner(dir.path()).unwrap().unwrap();
    assert_eq!(runner, "flutter");
    assert_eq!(args, ["test", "--no-pub"]);
}

#[test]
fn swift_tests_disable_package_updates_and_automatic_resolution() {
    let dir = tempfile::tempdir().unwrap();
    std::fs::write(
        dir.path().join("Package.swift"),
        "// swift-tools-version: 5.9\nlet package = Package(name: \"fixture\")\n",
    )
    .unwrap();

    let (runner, args) = detect_test_runner(dir.path()).unwrap().unwrap();
    assert_eq!(runner, "swift");
    assert_eq!(
        args,
        ["test", "--skip-update", "--disable-automatic-resolution"]
    );
}

#[test]
fn rust_and_go_tests_are_locked_offline() {
    let rust = tempfile::tempdir().unwrap();
    std::fs::write(
        rust.path().join("Cargo.toml"),
        "[package]\nname='fixture'\nversion='0.1.0'\nedition='2024'\n",
    )
    .unwrap();
    let (runner, args) = detect_test_runner(rust.path()).unwrap().unwrap();
    assert_eq!(runner, "cargo");
    assert!(args.contains(&"--locked".to_string()));
    assert!(args.contains(&"--offline".to_string()));

    let go = tempfile::tempdir().unwrap();
    std::fs::write(
        go.path().join("go.mod"),
        "module example.test/fixture\n\ngo 1.22\n",
    )
    .unwrap();
    let (runner, args) = detect_test_runner(go.path()).unwrap().unwrap();
    assert_eq!(runner, "go");
    assert!(args.contains(&"-mod=readonly".to_string()));
}

#[test]
fn python_tests_run_from_the_mgc_import_path_without_system_site_packages() {
    let dir = tempfile::tempdir().unwrap();
    std::fs::write(
        dir.path().join("pyproject.toml"),
        "[project]\nname='fixture'\nversion='0.1.0'\ndependencies=[]\n",
    )
    .unwrap();

    let (runner, args) = detect_test_runner(dir.path()).unwrap().unwrap();
    assert!(runner == "python" || runner == "python3");
    assert_eq!(args, ["-S", "-m", "pytest", "-s", "-v"]);
}

#[test]
fn core_detection_uses_shared_marker_parser_and_accepts_its_comment_format() {
    let dir = tempfile::tempdir().unwrap();
    std::fs::write(dir.path().join(".mgc.core"), "app # project identity\n").unwrap();
    std::fs::write(dir.path().join("package.json"), "{}").unwrap();

    assert_eq!(detect_test_core(dir.path()).unwrap(), "app");
}

#[test]
fn invalid_core_marker_fails_closed_instead_of_falling_back_to_manifest() {
    let dir = tempfile::tempdir().unwrap();
    std::fs::write(dir.path().join(".mgc.core"), "not-a-core\n").unwrap();
    std::fs::write(dir.path().join("package.json"), "{}").unwrap();

    let error = detect_test_core(dir.path()).unwrap_err();
    assert!(
        error.to_string().contains("invalid core marker")
            && !error.to_string().contains("signature")
    );
}

#[cfg(unix)]
#[test]
fn symlinked_core_marker_is_rejected_by_test_runtime_detection() {
    let dir = tempfile::tempdir().unwrap();
    let outside = dir.path().join("outside-marker");
    std::fs::write(&outside, "web\n").unwrap();
    std::os::unix::fs::symlink(&outside, dir.path().join(".mgc.core")).unwrap();

    assert!(detect_test_core(dir.path()).is_err());
}

#[cfg(unix)]
#[test]
fn symlinked_test_manifests_are_not_read_or_forwarded() {
    let dir = tempfile::tempdir().unwrap();
    let outside = tempfile::tempdir().unwrap();
    std::fs::write(
        outside.path().join("package.json"),
        r#"{"scripts":{"test":"echo external"}}"#,
    )
    .unwrap();
    std::os::unix::fs::symlink(
        outside.path().join("package.json"),
        dir.path().join("package.json"),
    )
    .unwrap();

    assert!(detect_test_runner(dir.path()).is_err());

    std::fs::remove_file(dir.path().join("package.json")).unwrap();
    std::fs::write(
        outside.path().join("mgc.toml"),
        "[scripts]\ntest = \"echo outside\"\n",
    )
    .unwrap();
    std::os::unix::fs::symlink(outside.path().join("mgc.toml"), dir.path().join("mgc.toml"))
        .unwrap();
    assert!(resolve_mgc_toml_script(&dir.path().join("mgc.toml"), "test").is_err());
}

#[cfg(unix)]
#[test]
fn symlinked_swift_manifest_is_rejected_before_runner_selection() {
    let dir = tempfile::tempdir().unwrap();
    let outside = tempfile::tempdir().unwrap();
    std::fs::write(
        outside.path().join("Package.swift"),
        "// swift-tools-version: 5.9\nlet package = Package(name: \"outside\")\n",
    )
    .unwrap();
    std::os::unix::fs::symlink(
        outside.path().join("Package.swift"),
        dir.path().join("Package.swift"),
    )
    .unwrap();

    assert!(detect_test_runner(dir.path()).is_err());
}

#[cfg(unix)]
#[test]
fn symlinked_legacy_manifest_is_rejected_for_test_core_detection() {
    let dir = tempfile::tempdir().unwrap();
    let outside = tempfile::tempdir().unwrap();
    std::fs::write(
        outside.path().join("pyproject.toml"),
        "[project]\ndependencies = [\"torch\"]\n",
    )
    .unwrap();
    std::os::unix::fs::symlink(
        outside.path().join("pyproject.toml"),
        dir.path().join("pyproject.toml"),
    )
    .unwrap();
    assert!(detect_test_core(dir.path()).is_err());
}

// ── Java and .NET test lanes ────────────────────────────────────────────
// `mgc build`/`mgc install` supported these languages while `mgc test` claimed
// no runner existed; these cases pin the new lanes and their offline flags.
// (`mgc build`/`install` hỗ trợ hai ngôn ngữ này trong khi `mgc test` báo không
// có runner; các case này ghim lane mới và cờ offline.)

#[test]
fn maven_tests_run_offline_against_the_mgc_local_repository() {
    let dir = tempfile::tempdir().unwrap();
    std::fs::write(
        dir.path().join("pom.xml"),
        "<project><modelVersion>4.0.0</modelVersion></project>",
    )
    .unwrap();

    let (runner, args) = detect_test_runner(dir.path()).unwrap().unwrap();
    assert_eq!(runner, "mvn");
    assert_eq!(args, ["-o", "test"], "Maven must not reach the network");
}

#[test]
fn both_gradle_dsls_select_gradle_offline() {
    for manifest in ["build.gradle", "build.gradle.kts"] {
        let dir = tempfile::tempdir().unwrap();
        std::fs::write(dir.path().join(manifest), "plugins { id 'java' }").unwrap();

        let (runner, args) = detect_test_runner(dir.path()).unwrap().unwrap();
        assert_eq!(runner, "gradle", "{manifest}");
        assert_eq!(args, ["--offline", "test"], "{manifest} must run offline");
    }
}

#[test]
fn a_dotnet_project_tests_without_restoring() {
    let dir = tempfile::tempdir().unwrap();
    std::fs::write(
        dir.path().join("Fixture.csproj"),
        "<Project Sdk=\"Microsoft.NET.Sdk\"></Project>",
    )
    .unwrap();

    let (runner, args) = detect_test_runner(dir.path()).unwrap().unwrap();
    assert_eq!(runner, "dotnet");
    assert_eq!(
        args,
        ["test", "--no-restore", "--nologo"],
        "MagiCore already restored; dotnet must not restore again"
    );
}

/// A solution file is enough: .NET test discovery works from the `.sln`.
/// (File solution là đủ: dotnet test tìm được từ `.sln`.)
#[test]
fn a_dotnet_solution_also_selects_dotnet() {
    let dir = tempfile::tempdir().unwrap();
    std::fs::write(
        dir.path().join("Fixture.sln"),
        "Microsoft Visual Studio Solution File",
    )
    .unwrap();

    let (runner, _args) = detect_test_runner(dir.path()).unwrap().unwrap();
    assert_eq!(runner, "dotnet");
}

/// The extension match ignores case, because Windows tooling does.
/// (So khớp extension không phân biệt hoa thường vì tooling Windows như vậy.)
#[test]
fn the_dotnet_extension_match_ignores_case() {
    let dir = tempfile::tempdir().unwrap();
    std::fs::write(dir.path().join("Fixture.CSPROJ"), "<Project></Project>").unwrap();

    let (runner, _args) = detect_test_runner(dir.path()).unwrap().unwrap();
    assert_eq!(runner, "dotnet");
}

/// A directory named `something.csproj` is not a project file.
/// (Thư mục tên `something.csproj` không phải project file.)
#[test]
fn a_directory_that_looks_like_a_project_file_is_ignored() {
    let dir = tempfile::tempdir().unwrap();
    std::fs::create_dir(dir.path().join("Trap.csproj")).unwrap();

    assert!(
        detect_test_runner(dir.path()).unwrap().is_none(),
        "a directory must not be mistaken for a .csproj"
    );
}

/// No manifest means no runner, and the caller reports it rather than guessing.
/// (Không manifest nghĩa là không runner; caller báo thay vì đoán.)
#[test]
fn an_empty_project_has_no_runner() {
    let dir = tempfile::tempdir().unwrap();
    assert!(detect_test_runner(dir.path()).unwrap().is_none());
}

/// Every language `mgc build` supports must have a test lane.
///
/// This is the parity rule stated as a test: the seven build manifests each
/// resolve to a runner, so no language can be built but not tested.
/// (Quy tắc parity dạng test: bảy manifest build đều quy ra một runner, nên
/// không ngôn ngữ nào build được mà không test được.)
#[test]
fn every_buildable_language_has_a_test_runner() {
    let cases: &[(&str, &str, &str)] = &[
        (
            "Cargo.toml",
            "[package]\nname=\"x\"\nversion=\"0.1.0\"\n",
            "cargo",
        ),
        ("go.mod", "module x\n", "go"),
        ("pyproject.toml", "[project]\nname=\"x\"\n", "python3"),
        ("pubspec.yaml", "name: x\n", "flutter"),
        ("Package.swift", "// swift-tools-version: 5.9\n", "swift"),
        ("pom.xml", "<project></project>", "mvn"),
        ("build.gradle", "plugins { }", "gradle"),
        ("Fixture.csproj", "<Project></Project>", "dotnet"),
    ];
    for (manifest, body, expected) in cases {
        let dir = tempfile::tempdir().unwrap();
        std::fs::write(dir.path().join(manifest), body).unwrap();
        let detected = detect_test_runner(dir.path())
            .unwrap()
            .unwrap_or_else(|| panic!("{manifest} must select a test runner"));
        // Python's launcher is `python_cmd()` (python3 on unix, python on
        // Windows), so assert the lane rather than the literal binary name.
        // (Launcher Python là `python_cmd()` nên chỉ khẳng định lane.)
        let matches =
            detected.0 == *expected || (*expected == "python3" && detected.0.starts_with("python"));
        assert!(
            matches,
            "{manifest} selected {} instead of {expected}",
            detected.0
        );
    }
}
