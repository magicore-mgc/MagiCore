//! Native Remove lifecycle E2E cases. / Test E2E lifecycle cho native remove.

use crate::native_ownership_matrix_e2e::support::{
    HermeticMaven, HermeticNuget, HermeticPypi, bevy_with_dep, blocked_mutation_unchanged,
    dotnet_manifest, esp32_with_dep, flutter_with_meta, go_manifest, java_manifest,
    kotlin_with_dep, lib_project, native_remove_cell, native_remove_cell_with_env,
    native_remove_only, native_remove_only_with_env, python_manifest, rust_manifest,
    swift_with_dep, write,
};
use tempfile::TempDir;

pub(crate) fn matrix_native_remove_all_lanes() {
    let py = |d: &std::path::Path| {
        let (f, b) = python_manifest();
        lib_project(d, "python", f, &b);
    };
    let pypi = HermeticPypi::new(&[("six", "1.17.0")]);
    let python_env = [("MGC_PYPI_INDEX_URL", pypi.url.as_str())];
    native_remove_cell_with_env(
        py,
        &["add-lib", "six"],
        &["remove-lib", "six"],
        "pyproject.toml",
        "six",
        &python_env,
    );
    let rs = |d: &std::path::Path| {
        let (f, b) = rust_manifest();
        lib_project(d, "rust", f, &b);
    };
    native_remove_cell(
        rs,
        &["add-lib", "serde_json"],
        &["remove-lib", "serde_json"],
        "Cargo.toml",
        "serde_json",
    );
    let go = |d: &std::path::Path| {
        let (f, b) = go_manifest();
        lib_project(d, "go", f, &b);
    };
    native_remove_cell(
        go,
        &["add-lib", "github.com/google/uuid"],
        &["remove-lib", "github.com/google/uuid"],
        "go.mod",
        "github.com/google/uuid",
    );
    let net = |d: &std::path::Path| {
        let (f, b) = dotnet_manifest();
        lib_project(d, "dotnet", f, &b);
    };
    let nuget = HermeticNuget::new();
    let nuget_env = [("MGC_NUGET_INDEX_URL", nuget.index_url.as_str())];
    native_remove_cell_with_env(
        net,
        &["add-lib", "Newtonsoft.Json"],
        &["remove-lib", "Newtonsoft.Json"],
        "m.csproj",
        "Newtonsoft.Json",
        &nuget_env,
    );
    nuget.assert_all_routes_used();
    let java = |d: &std::path::Path| {
        let (f, b) = java_manifest();
        lib_project(d, "java", f, &b);
    };
    let maven = HermeticMaven::new();
    let maven_env = [("MGC_MAVEN_REPO_URL", maven.url.as_str())];
    native_remove_cell_with_env(
        java,
        &["add-lib", "org.apache.commons:commons-lang3"],
        &["remove-lib", "org.apache.commons:commons-lang3"],
        "pom.xml",
        "commons-lang3",
        &maven_env,
    );
    maven.assert_all_routes_used();
}

pub(crate) fn matrix_native_remove_flutter_swift_kotlin() {
    // Flutter's journaled writer owns remove. Swift/Kotlin are blocked at
    // the C0 firewall until their writers join that transaction; never
    // label a refusal as native remove support.
    // (Flutter dùng writer có journal; Swift/Kotlin bị chặn tới khi có transaction.)
    let sdk = TempDir::new().unwrap();
    let flutter_package = sdk.path().join("packages/flutter");
    std::fs::create_dir_all(flutter_package.join("lib")).unwrap();
    write(&flutter_package, "pubspec.yaml", "name: flutter\n");
    let sdk_root = sdk.path().to_string_lossy().to_string();
    native_remove_only_with_env(
        flutter_with_meta,
        &["remove-app", "meta"],
        "pubspec.yaml",
        "meta",
        &[("FLUTTER_ROOT", &sdk_root)],
    );
    blocked_mutation_unchanged(
        swift_with_dep,
        &["remove-app", "scope/lib"],
        "Package.swift",
    );
    blocked_mutation_unchanged(
        kotlin_with_dep,
        &["remove-app", "commons-lang3"],
        "gradle/libs.versions.toml",
    );
}

pub(crate) fn matrix_native_remove_game_iot() {
    native_remove_only(
        bevy_with_dep,
        &["remove-game", "serde_json"],
        "Cargo.toml",
        "serde_json",
    );
    native_remove_only(
        esp32_with_dep,
        &["remove-iot", "serde_json"],
        "Cargo.toml",
        "serde_json",
    );
}
