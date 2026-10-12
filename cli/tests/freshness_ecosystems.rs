//! Cross-ecosystem freshness tests — kiểm thử freshness đa ecosystem.
//!
//! Manifest projections (python/flutter/cargo/go) plus lock comparison
//! (Spec rev4 §8/§15 task 7). Script-adjacent edits never participate.
//! (Projection manifest (python/flutter/cargo/go) cộng so lock
//! (Spec rev4 §8/§15 task 7). Sửa quanh scripts không tham gia.)

#![allow(clippy::unwrap_used)]

use mgc::commands::freshness::{
    check_cargo_freshness, check_flutter_freshness, check_python_freshness,
    check_web_go_build_freshness, declared_cargo_deps, declared_flutter_deps, declared_go_deps,
    declared_python_deps,
};

type FreshnessChecker = fn(&std::path::Path) -> anyhow::Result<()>;

fn lock_with(packages: &[(&str, &str)]) -> String {
    lock_with_owner(packages, "web", mgc_lockfile::EcosystemTag::Web)
}

fn lock_with_owner(
    packages: &[(&str, &str)],
    owner: &str,
    ecosystem: mgc_lockfile::EcosystemTag,
) -> String {
    let mut lock = mgc_lockfile::Lockfile::new();
    for (name, version) in packages {
        let mut package = mgc_lockfile::Package::new(
            (*name).into(),
            (*version).into(),
            format!("https://example.invalid/{name}-{version}"),
            "sha256-test-integrity".to_string(),
        );
        package.owner_core = Some(owner.to_string());
        package.ecosystem = ecosystem;
        lock.add_package(package);
    }
    mgc_lockfile::update_owner_root_pins(
        &mut lock,
        owner,
        ecosystem,
        packages
            .iter()
            .map(|(name, version)| format!("{name}@{version}")),
    );
    mgc_lockfile::writer::serialize_lockfile(&lock).unwrap()
}

fn polyglot_lock() -> String {
    let mut lock = mgc_lockfile::Lockfile::new();
    for (owner, ecosystem, packages) in [
        (
            "web",
            mgc_lockfile::EcosystemTag::Web,
            &[("react", "18.0.0")][..],
        ),
        (
            "ai",
            mgc_lockfile::EcosystemTag::Python,
            &[("requests", "2.0.0")][..],
        ),
        (
            "lib",
            mgc_lockfile::EcosystemTag::Python,
            &[("requests", "2.0.0"), ("other-lib-root", "1.0.0")][..],
        ),
    ] {
        for (name, version) in packages {
            let mut package = mgc_lockfile::Package::new(
                (*name).into(),
                (*version).into(),
                format!("https://example.invalid/{name}-{version}"),
                "sha256-test-integrity".to_string(),
            );
            package.owner_core = Some(owner.into());
            package.ecosystem = ecosystem;
            lock.add_package(package);
        }
        mgc_lockfile::update_owner_root_pins(
            &mut lock,
            owner,
            ecosystem,
            packages
                .iter()
                .map(|(name, version)| format!("{name}@{version}")),
        );
    }
    mgc_lockfile::writer::serialize_lockfile(&lock).unwrap()
}

// PEP 508 / PEP 503 normalization.
// (Chuẩn hóa PEP 508 / PEP 503.)
#[test]
fn python_projection() {
    let deps = declared_python_deps(
        "[project]\nname = \"a\"\ndependencies = [\"Foo_Bar>=1.0\", \"requests\", \"uvicorn[standard]>=0.20; python_version >= '3.11'\"]\n",
    );
    assert_eq!(deps, vec!["foo-bar", "requests", "uvicorn"]);
    let broken = declared_python_deps("[[[invalid");
    assert!(broken.is_empty());
}

// pubspec top-level dependency mapping.
// (Mapping dependency top-level pubspec.)
#[test]
fn flutter_projection() {
    let deps = declared_flutter_deps(
        "name: a\ndependencies:\n  flutter:\n    sdk: flutter\n  http: ^1.0.0\ndev_dependencies:\n  test: any\n",
    );
    assert_eq!(deps, vec!["flutter", "http", "test"]);
}

// Cargo tables including target-specific ones.
// (Bảng Cargo kể cả theo target.)
#[test]
fn cargo_projection() {
    let deps = declared_cargo_deps(
        "[package]\nname = \"a\"\n[dependencies]\nserde = \"1\"\n[dev-dependencies]\nfoo = \"1\"\n[target.'cfg(unix)'.dependencies]\nbar = \"1\"\n",
    );
    assert_eq!(deps, vec!["bar", "foo", "serde"]);
}

// go.mod require blocks and single lines.
// (Khối require và dòng lẻ go.mod.)
#[test]
fn go_projection() {
    let deps = declared_go_deps(
        "module example.com/a\n\ngo 1.21\n\nrequire (\n\tgithub.com/x/y v1.0.0\n)\n\nrequire golang.org/x/mod v0.1.0 // indirect\n",
    );
    assert_eq!(deps, vec!["github.com/x/y", "golang.org/x/mod"]);
}

// Python drift fails closed; match passes.
// (Lệch Python fail-closed; khớp thì qua.)
#[test]
fn python_freshness() {
    let dir = tempfile::TempDir::new().unwrap();
    std::fs::write(
        dir.path().join("pyproject.toml"),
        "[project]\nname = \"a\"\ndependencies = [\"requests\", \"missing-pkg\"]\n",
    )
    .unwrap();
    std::fs::write(
        dir.path().join("mgc.lock"),
        lock_with_owner(
            &[("requests", "2.0.0")],
            "ai",
            mgc_lockfile::EcosystemTag::Python,
        ),
    )
    .unwrap();
    let err = check_python_freshness(dir.path()).unwrap_err();
    assert!(err.to_string().contains("missing-pkg"));

    std::fs::write(
        dir.path().join("mgc.lock"),
        lock_with_owner(
            &[("requests", "2.0.0"), ("missing-pkg", "1.0.0")],
            "ai",
            mgc_lockfile::EcosystemTag::Python,
        ),
    )
    .unwrap();
    check_python_freshness(dir.path()).unwrap();
}

// Native dev checks require a lock when their ecosystem dependency manifest
// exists, including an empty manifest; unrelated lanes without that manifest
// do not inherit a false lock requirement.
// Dev native cần lock khi manifest dependency của ecosystem hiện diện, kể cả
// manifest rỗng; lane không liên quan không bị ép có lock.
#[test]
fn ecosystem_dev_freshness_requires_lock_for_empty_manifest() {
    let checks: [(&str, &str, FreshnessChecker); 3] = [
        (
            "pyproject.toml",
            "[project]\nname='a'\n",
            check_python_freshness,
        ),
        (
            "pubspec.yaml",
            "name: a\ndependencies: {}\n",
            check_flutter_freshness,
        ),
        (
            "Cargo.toml",
            "[package]\nname='a'\nversion='0.1.0'\n",
            check_cargo_freshness,
        ),
    ];
    for (file, content, check) in checks {
        let dir = tempfile::TempDir::new().unwrap();
        std::fs::write(dir.path().join(file), content).unwrap();
        let err = check(dir.path()).unwrap_err();
        assert!(
            err.to_string().contains("mgc.lock"),
            "unexpected error: {err}"
        );
    }
    let dir = tempfile::TempDir::new().unwrap();
    std::fs::write(
        dir.path().join("Package.swift"),
        "// swift-tools-version: 5.9\n",
    )
    .unwrap();
    let err = mgc::commands::freshness::check_swift_freshness(dir.path()).unwrap_err();
    assert!(
        err.to_string().contains("mgc.lock"),
        "unexpected error: {err}"
    );
}

// Syntactically invalid TOML fails before its empty projection can pass.
// (TOML sai cú pháp bị chặn trước khi projection rỗng có thể qua.)
#[test]
fn malformed_python_manifest_fails_closed() {
    let dir = tempfile::TempDir::new().unwrap();
    std::fs::write(dir.path().join("pyproject.toml"), "[[[invalid\n").unwrap();
    std::fs::write(
        dir.path().join("mgc.lock"),
        lock_with_owner(
            &[("requests", "2.0.0")],
            "ai",
            mgc_lockfile::EcosystemTag::Python,
        ),
    )
    .unwrap();

    let err = check_python_freshness(dir.path()).unwrap_err();
    assert!(
        err.to_string().contains("valid TOML"),
        "unexpected error: {err}"
    );
}

// Valid TOML with an invalid dependency-section type cannot project to empty.
// (TOML hợp lệ nhưng section dependency sai kiểu không được chiếu thành rỗng.)
#[test]
fn malformed_python_dependency_shape_fails_closed() {
    let dir = tempfile::TempDir::new().unwrap();
    std::fs::write(
        dir.path().join("pyproject.toml"),
        "[project]\nname = \"a\"\ndependencies = \"requests\"\n",
    )
    .unwrap();
    std::fs::write(
        dir.path().join("mgc.lock"),
        lock_with_owner(
            &[("requests", "2.0.0")],
            "ai",
            mgc_lockfile::EcosystemTag::Python,
        ),
    )
    .unwrap();

    let err = check_python_freshness(dir.path()).unwrap_err();
    assert!(
        err.to_string().contains("array of strings"),
        "unexpected error: {err}"
    );
}

// Flutter drift fails closed.
// (Lệch Flutter fail-closed.)
#[test]
fn flutter_freshness() {
    let dir = tempfile::TempDir::new().unwrap();
    std::fs::write(
        dir.path().join("pubspec.yaml"),
        "name: a\ndependencies:\n  http: ^1.0.0\n",
    )
    .unwrap();
    std::fs::write(dir.path().join("mgc.lock"), lock_with(&[])).unwrap();
    let err = check_flutter_freshness(dir.path()).unwrap_err();
    assert!(err.to_string().contains("http"));
}

// Cargo drift fails closed; absent manifests are a no-op.
// (Lệch Cargo fail-closed; manifest vắng là no-op.)
#[test]
fn cargo_freshness() {
    let dir = tempfile::TempDir::new().unwrap();
    std::fs::write(
        dir.path().join("Cargo.toml"),
        "[package]\nname = \"a\"\n[dependencies]\nserde = \"1\"\n",
    )
    .unwrap();
    std::fs::write(dir.path().join("mgc.lock"), lock_with(&[])).unwrap();
    assert!(check_cargo_freshness(dir.path()).is_err());

    let empty = tempfile::TempDir::new().unwrap();
    check_cargo_freshness(empty.path()).unwrap();
    check_python_freshness(empty.path()).unwrap();
    check_flutter_freshness(empty.path()).unwrap();
}

// Removing a direct dependency is stale across Python, Flutter, and Cargo.
// Xóa dependency trực tiếp được phát hiện ở Python, Flutter và Cargo.
#[test]
fn removed_cross_ecosystem_dependencies_fail_closed() {
    let cases: [(
        &str,
        &str,
        &str,
        mgc_lockfile::EcosystemTag,
        FreshnessChecker,
    ); 3] = [
        (
            "pyproject.toml",
            "[project]\nname='a'\ndependencies=['kept']\n",
            "ai",
            mgc_lockfile::EcosystemTag::Python,
            check_python_freshness,
        ),
        (
            "pubspec.yaml",
            "name: a\ndependencies:\n  kept: ^1.0.0\n",
            "app",
            mgc_lockfile::EcosystemTag::Dart,
            check_flutter_freshness,
        ),
        (
            "Cargo.toml",
            "[package]\nname='a'\nversion='0.1.0'\n[dependencies]\nkept='1'\n",
            "game",
            mgc_lockfile::EcosystemTag::Rust,
            check_cargo_freshness,
        ),
    ];
    for (manifest, content, owner, ecosystem, check) in cases {
        let dir = tempfile::TempDir::new().unwrap();
        std::fs::write(dir.path().join(manifest), content).unwrap();
        std::fs::write(
            dir.path().join("mgc.lock"),
            lock_with_owner(&[("kept", "1.0.0"), ("removed", "1.0.0")], owner, ecosystem),
        )
        .unwrap();
        let err = check(dir.path()).unwrap_err();
        assert!(
            err.to_string().contains("removed"),
            "{manifest}: unexpected error: {err}"
        );
    }
}

// Framework resolution binds id + locked version for profile lookup.
// (Xác định framework gắn id + version lock để tra profile.)
#[test]
fn web_framework_resolution() {
    use mgc::commands::freshness::resolve_web_framework;
    let dir = tempfile::TempDir::new().unwrap();
    std::fs::write(
        dir.path().join("package.json"),
        r#"{"dependencies": {"next": "16.3.8", "react": "19.0.0"}}"#,
    )
    .unwrap();
    std::fs::write(
        dir.path().join("mgc.lock"),
        lock_with(&[("next", "16.3.8"), ("react", "19.0.0")]),
    )
    .unwrap();
    assert_eq!(
        resolve_web_framework(dir.path()).unwrap(),
        Some(("next".to_string(), "16.3.8".to_string()))
    );

    let plain = tempfile::TempDir::new().unwrap();
    std::fs::write(
        plain.path().join("package.json"),
        r#"{"dependencies": {"left-pad": "1.3.0"}}"#,
    )
    .unwrap();
    assert_eq!(resolve_web_framework(plain.path()).unwrap(), None);

    let ambiguous = tempfile::TempDir::new().unwrap();
    std::fs::write(
        ambiguous.path().join("package.json"),
        r#"{"dependencies":{"next":"16.3.8","vite":"7.0.0"}}"#,
    )
    .unwrap();
    std::fs::write(
        ambiguous.path().join("mgc.lock"),
        lock_with(&[("next", "16.3.8"), ("vite", "7.0.0")]),
    )
    .unwrap();
    assert_eq!(resolve_web_framework(ambiguous.path()).unwrap(), None);
}

// Profile gate: requires_pm refuses upfront; absent profile passes through.
// (Cổng profile: requires_pm từ chối upfront; vắng profile thì qua.)
#[test]
fn profile_gate() {
    use mgc::commands::freshness::{check_profile_gate, default_profile_store_dir};
    let _ = default_profile_store_dir();

    let dir = tempfile::TempDir::new().unwrap();
    std::fs::write(
        dir.path().join("package.json"),
        r#"{"dependencies": {"next": "16.3.8"}}"#,
    )
    .unwrap();
    let lock = lock_with(&[("next", "16.3.8")]);
    std::fs::write(dir.path().join("mgc.lock"), &lock).unwrap();

    // No profile anywhere reachable → level-2 pass-through.
    // (Không profile nào tới được → mức 2 cho qua.)
    let empty_store = tempfile::TempDir::new().unwrap();
    check_profile_gate(dir.path(), empty_store.path(), "dev", None).unwrap();

    // A profile demanding npm refuses before spawn.
    // (Profile đòi npm thì từ chối trước spawn.)
    let digest = mgc_crypto::blake3_signer::Blake3Hasher::hash_bytes(lock.as_bytes()).to_hex();
    let profile = serde_json::json!({
        "schema_version": "spawn-profile-v1",
        "mgc_version": "1.1.0-rc.9",
        "corpus_id": "test",
        "lock_digest": digest,
        "captured_at": 1,
        "profile": {
            "framework": "next",
            "version": "16.3.8",
            "os": std::env::consts::OS,
            "operation": "dev",
            "allowed_pm_invocations": [],
            "requires_pm": ["npm"]
        }
    });
    std::fs::write(
        empty_store
            .path()
            .join(format!("next-dev-{}.json", std::env::consts::OS)),
        serde_json::to_string(&profile).unwrap(),
    )
    .unwrap();
    let err = check_profile_gate(dir.path(), empty_store.path(), "dev", None).unwrap_err();
    assert!(err.to_string().contains("--compat-runner"), "got: {err}");
    check_profile_gate(dir.path(), empty_store.path(), "dev", Some("npm")).unwrap();
    let wrong_runner =
        check_profile_gate(dir.path(), empty_store.path(), "dev", Some("pnpm")).unwrap_err();
    assert!(
        wrong_runner
            .to_string()
            .contains("requires package managers"),
        "unexpected error: {wrong_runner}"
    );
}

// Maven artifactIds inside the top-level dependencies block.
// (artifactId Maven trong khối dependencies top-level.)
#[test]
fn maven_projection() {
    use mgc::commands::freshness::declared_maven_deps;
    let deps = declared_maven_deps(
        "<project><groupId>com.demo</groupId><artifactId>app</artifactId>\
         <dependencies><dependency><groupId>com.demo</groupId>\
         <artifactId>core</artifactId><version>1.0.0</version></dependency>\
         <dependency><groupId>org.x</groupId><artifactId>lib</artifactId>\
         <version>2.0.0</version></dependency></dependencies></project>",
    );
    assert_eq!(deps, vec!["com.demo:core", "org.x:lib"]);
    assert!(declared_maven_deps("<project></project>").is_empty());
}

// Gradle implementation-family coordinates.
// (Tọa độ họ implementation Gradle.)
#[test]
fn gradle_projection() {
    use mgc::commands::freshness::declared_gradle_deps;
    let deps = declared_gradle_deps(
        "plugins { id 'java' }\ndependencies {\n    implementation(\"com.demo:core:1.0.0\")\n    testImplementation 'org.x:lib:2.0.0'\n    implementation project(':local')\n}\n",
    );
    assert_eq!(deps, vec!["com.demo:core:1.0.0", "org.x:lib:2.0.0"]);
}

// NuGet PackageReference ids.
// (Id PackageReference NuGet.)
#[test]
fn nuget_projection() {
    use mgc::commands::freshness::declared_nuget_deps;
    let deps = declared_nuget_deps(
        "<Project Sdk=\"Microsoft.NET.Sdk\"><ItemGroup>\
         <PackageReference Include=\"Demo.Lib\" Version=\"1.2.3\" />\
         <PackageReference Include=\"Other.Pkg\" Version=\"4.5.6\" />\
         </ItemGroup></Project>",
    );
    assert_eq!(deps, vec!["Demo.Lib", "Other.Pkg"]);
}

// Swift package URL identities.
// (Identity URL package Swift.)
#[test]
fn swift_projection() {
    use mgc::commands::freshness::declared_swift_deps;
    let deps = declared_swift_deps(
        "let package = Package(dependencies: [.package(url: \"https://github.com/org/Kit.git\", from: \"1.0.0\"), .package(url: \"https://example.com/Tool\", exact: \"2.0.0\"), .package(id: \"Scope.Name\", version: \"1.0.0\")])",
    );
    assert_eq!(deps, vec!["kit", "scope.name", "tool"]);
}

// Unity UPM dependency ids.
// (Id dependency UPM Unity.)
#[test]
fn unity_projection() {
    use mgc::commands::freshness::declared_unity_deps;
    let deps = declared_unity_deps(
        r#"{"dependencies": {"com.unity.nuget.newtonsoft-json": "3.2.1", "com.demo.tool": "1.0.0"}, "scopedRegistries": []}"#,
    );
    assert_eq!(
        deps,
        vec!["com.demo.tool", "com.unity.nuget.newtonsoft-json"]
    );
}

// PlatformIO lib_deps tokens.
// (Token lib_deps PlatformIO.)
#[test]
fn platformio_projection() {
    use mgc::commands::freshness::declared_platformio_deps;
    let deps = declared_platformio_deps(
        "[env:esp32]\nplatform = espressif32\nlib_deps =\n  bblanchon/ArduinoJson @ ^6.21.0\n  https://github.com/user/lib.zip\n",
    );
    assert_eq!(
        deps,
        vec![
            "^6.21.0".to_string(),
            "bblanchon/ArduinoJson".to_string(),
            "https://github.com/user/lib.zip".to_string(),
        ]
    );
}

// West manifest project names.
// (Tên project manifest west.)
#[test]
fn west_projection() {
    use mgc::commands::freshness::declared_west_deps;
    let deps = declared_west_deps(
        "manifest:\n  projects:\n    - name: zephyr\n      remote: upstream\n    - name: hal_nordic\n",
    );
    assert_eq!(deps, vec!["hal_nordic", "zephyr"]);
}

// Swift freshness end-to-end against a lock (lowercase identities).
// (Freshness Swift đối chiếu lock (identity lowercase).)
#[test]
fn swift_freshness() {
    use mgc::commands::freshness::check_swift_freshness;
    let dir = tempfile::TempDir::new().unwrap();
    std::fs::write(
        dir.path().join("Package.swift"),
        "let package = Package(dependencies: [.package(url: \"https://github.com/org/Kit.git\", from: \"1.0.0\")])",
    )
    .unwrap();
    std::fs::write(
        dir.path().join("mgc.lock"),
        lock_with(&[("other", "1.0.0")]),
    )
    .unwrap();
    let err = check_swift_freshness(dir.path()).unwrap_err();
    assert!(err.to_string().contains("kit"), "got: {err}");

    std::fs::write(
        dir.path().join("mgc.lock"),
        lock_with_owner(
            &[("kit", "1.0.0")],
            "app",
            mgc_lockfile::EcosystemTag::Swift,
        ),
    )
    .unwrap();
    check_swift_freshness(dir.path()).unwrap();
}

// Union check passes a matching polyglot project and fails drift.
// (Check hợp qua project đa ngôn ngữ khớp và fail khi lệch.)
#[test]
fn any_freshness_union() {
    use mgc::commands::freshness::check_any_freshness;
    let dir = tempfile::TempDir::new().unwrap();
    std::fs::write(
        dir.path().join("package.json"),
        r#"{"dependencies": {"react": "18.0.0"}}"#,
    )
    .unwrap();
    std::fs::write(
        dir.path().join("pyproject.toml"),
        "[project]\nname = \"a\"\ndependencies = [\"requests\"]\n",
    )
    .unwrap();
    std::fs::write(dir.path().join("mgc.lock"), polyglot_lock()).unwrap();
    check_any_freshness(dir.path()).unwrap();

    std::fs::write(
        dir.path().join("pyproject.toml"),
        "[project]\nname = \"a\"\ndependencies = [\"requests\", \"new-dep\"]\n",
    )
    .unwrap();
    let err = check_any_freshness(dir.path()).unwrap_err();
    assert!(err.to_string().contains("new-dep"), "got: {err}");
}

// Quoted go module paths are unwrapped.
// (Đường dẫn module go có quote được gỡ.)
#[test]
fn go_quoted_paths() {
    use mgc::commands::freshness::declared_go_deps;
    let deps = declared_go_deps("module a\n\ngo 1.21\n\nrequire \"gopkg.in/check.v1\" v1.0.0\n");
    assert_eq!(deps, vec!["gopkg.in/check.v1"]);
}

#[test]
fn go_build_requires_exact_web_owned_lock_pins() {
    let dir = tempfile::TempDir::new().unwrap();
    let manifest =
        "module example.test/backend\n\ngo 1.23\n\nrequire github.com/gin-gonic/gin v1.10.0\n";
    std::fs::write(dir.path().join("go.mod"), manifest).unwrap();

    let missing_lock = check_web_go_build_freshness(dir.path()).unwrap_err();
    assert!(missing_lock.to_string().contains("mgc.lock is missing"));

    std::fs::write(
        dir.path().join("mgc.lock"),
        lock_with_owner(
            &[("github.com/gin-gonic/gin", "1.10.0")],
            "web",
            mgc_lockfile::EcosystemTag::Go,
        ),
    )
    .unwrap();
    check_web_go_build_freshness(dir.path()).unwrap();

    std::fs::write(
        dir.path().join("go.mod"),
        manifest.replace("v1.10.0", "v1.11.0"),
    )
    .unwrap();
    let drift = check_web_go_build_freshness(dir.path()).unwrap_err();
    assert!(
        drift
            .to_string()
            .contains("exact declared version '1.11.0'")
    );
}

#[test]
fn go_build_rejects_replace_directives_without_a_verified_projection() {
    let dir = tempfile::TempDir::new().unwrap();
    std::fs::write(
        dir.path().join("go.mod"),
        "module example.test/backend\n\ngo 1.23\n\nrequire github.com/gin-gonic/gin v1.10.0\nreplace github.com/gin-gonic/gin => ../gin\n",
    )
    .unwrap();

    let error = check_web_go_build_freshness(dir.path()).unwrap_err();
    assert!(error.to_string().contains("replace directives"));
}

// Lockless projects pass the union check (test must run without a lock).
// (Project chưa lock qua check hợp (test phải chạy được khi chưa lock).)
#[test]
fn any_freshness_lockless_passes() {
    use mgc::commands::freshness::check_any_freshness;
    let dir = tempfile::TempDir::new().unwrap();
    std::fs::write(
        dir.path().join("pyproject.toml"),
        "[project]\nname = \"a\"\ndependencies = [\"requests\"]\n",
    )
    .unwrap();
    check_any_freshness(dir.path()).unwrap();
}
