//! Lib manifest format ownership and project selection tests.
//! Test nhận diện format manifest Lib và chọn project.

use super::support::*;

#[test]
fn native_manifest_format_is_reported_only_for_owned_files() {
    let root = tempfile::tempdir().unwrap();
    for (language, file, contents, expected) in [
        (LibLanguage::Ts, "package.json", "{}\n", "package-json"),
        (
            LibLanguage::Rust,
            "Cargo.toml",
            "[package]\nname='fixture'\nversion='0.1.0'\n",
            "cargo-toml",
        ),
        (
            LibLanguage::Go,
            "go.mod",
            "module example.test/fixture\n\ngo 1.24\n",
            "go-mod",
        ),
    ] {
        assert_eq!(dependency_manifest_format(root.path(), language), None);
        std::fs::write(root.path().join(file), contents).unwrap();
        assert_eq!(
            dependency_manifest_format(root.path(), language),
            Some(expected)
        );
        std::fs::remove_file(root.path().join(file)).unwrap();
    }

    std::fs::write(
        root.path().join("pyproject.toml"),
        "[project]\nname='fixture'\nversion='0.1.0'\ndependencies=[]\n",
    )
    .unwrap();
    assert_eq!(
        dependency_manifest_format(root.path(), LibLanguage::Python),
        Some("pep621-native")
    );
    std::fs::write(root.path().join("uv.lock"), "version = 1\n").unwrap();
    assert_eq!(
        dependency_manifest_format(root.path(), LibLanguage::Python),
        Some("pyproject-unsupported"),
        "foreign lock ownership must not be advertised as native"
    );
}
#[test]
fn java_native_format_is_maven_only_and_gradle_is_explicit() {
    let root = tempfile::tempdir().unwrap();
    assert_eq!(
        dependency_manifest_format(root.path(), LibLanguage::Java),
        None
    );

    std::fs::write(root.path().join("build.gradle.kts"), "plugins {}").unwrap();
    assert_eq!(
        dependency_manifest_format(root.path(), LibLanguage::Java),
        Some("gradle")
    );

    std::fs::write(root.path().join("pom.xml"), "<project/>").unwrap();
    assert_eq!(
        dependency_manifest_format(root.path(), LibLanguage::Java),
        Some("ambiguous"),
        "a mixed Maven/Gradle root must not silently select Maven"
    );
}
#[test]
fn java_native_format_selects_maven_when_gradle_is_absent() {
    let root = tempfile::tempdir().unwrap();
    std::fs::write(root.path().join("pom.xml"), "<project/>").unwrap();
    assert_eq!(
        dependency_manifest_format(root.path(), LibLanguage::Java),
        Some("maven-pom")
    );
}
#[test]
fn java_adapter_fails_closed_when_maven_and_gradle_are_both_present() {
    let root = tempfile::tempdir().unwrap();
    std::fs::write(root.path().join("pom.xml"), "<project/>").unwrap();
    std::fs::write(root.path().join("build.gradle"), "plugins {}").unwrap();
    let adapter = crate::adapter_for_language(LibLanguage::Java, root.path(), None, None).unwrap();

    let error = adapter.probe_dependency_resolver().unwrap_err();

    assert!(error.to_string().contains("both Maven and Gradle"));
}
#[test]
fn dotnet_native_format_requires_a_root_csproj_case_insensitively() {
    let root = tempfile::tempdir().unwrap();
    assert_eq!(
        dependency_manifest_format(root.path(), LibLanguage::DotNet),
        None
    );
    std::fs::write(root.path().join("Sample.CSPROJ"), "<Project/>").unwrap();
    assert_eq!(
        dependency_manifest_format(root.path(), LibLanguage::DotNet),
        Some("csproj")
    );
    std::fs::write(root.path().join("Second.csproj"), "<Project/>").unwrap();
    assert_eq!(
        dependency_manifest_format(root.path(), LibLanguage::DotNet),
        None,
        "multiple root projects require an explicit selector"
    );
}
#[test]
fn dotnet_target_framework_uses_the_same_case_insensitive_unique_csproj_gate() {
    let root = tempfile::tempdir().unwrap();
    std::fs::write(
        root.path().join("Sample.CSPROJ"),
        "<Project><PropertyGroup><TargetFramework>net9.0</TargetFramework></PropertyGroup></Project>",
    )
    .unwrap();
    assert_eq!(
        crate::adapter::read_dotnet_target_framework(root.path())
            .unwrap()
            .as_deref(),
        Some("net9.0")
    );

    std::fs::write(root.path().join("Other.csproj"), "<Project/>").unwrap();
    assert_eq!(
        crate::adapter::read_dotnet_target_framework(root.path()).unwrap(),
        None,
        "multiple project files must not make TFM selection diverge from ownership"
    );
}
#[cfg(unix)]
#[test]
fn dotnet_target_framework_refuses_symlinked_csproj() {
    use std::os::unix::fs::symlink;

    let root = tempfile::tempdir().unwrap();
    let outside = tempfile::tempdir().unwrap();
    let external_csproj = outside.path().join("External.csproj");
    std::fs::write(
        &external_csproj,
        "<Project><PropertyGroup><TargetFramework>net9.0</TargetFramework></PropertyGroup></Project>",
    )
    .unwrap();
    symlink(&external_csproj, root.path().join("External.csproj")).unwrap();

    assert!(
        crate::adapter::read_dotnet_target_framework_file(&root.path().join("External.csproj"))
            .is_err(),
        "target framework detection must not read a symlink outside the project"
    );
}
#[tokio::test]
async fn dotnet_resolve_gate_refuses_ambiguous_root_projects_before_network() {
    let root = tempfile::tempdir().unwrap();
    std::fs::write(root.path().join("App.csproj"), "<Project/>").unwrap();
    std::fs::write(root.path().join("Tests.csproj"), "<Project/>").unwrap();
    let adapter =
        crate::adapter_for_language(LibLanguage::DotNet, root.path(), None, None).unwrap();

    let probe_error = adapter.probe_dependency_resolver().unwrap_err();
    assert!(
        probe_error
            .to_string()
            .contains("exactly one regular root-level .csproj")
    );

    let resolve_error = adapter
        .resolve(&mgc_types::Manifest::new(
            "ambiguous",
            mgc_types::Ecosystem::Lib,
        ))
        .await
        .unwrap_err();
    assert!(
        resolve_error
            .to_string()
            .contains("exactly one regular root-level .csproj")
    );
}
