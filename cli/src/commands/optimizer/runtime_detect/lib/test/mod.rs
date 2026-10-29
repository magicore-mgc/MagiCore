//! Lib runtime detection tests — test nhận diện runtime lib.
//! Every language `mgc build --core lib` supports must yield a runtime, because
//! `mgc optimizer` refuses to run without one ("nothing was optimized, not a
//! success"). A language missing here has no optimizer at all.
//! (Mọi ngôn ngữ `mgc build --core lib` hỗ trợ phải trả runtime, vì `mgc
//! optimizer` từ chối chạy khi không có — ngôn ngữ thiếu ở đây là không có
//! optimizer.)

use super::detect_lib_runtime;
use crate::commands::optimizer::runtime_detect::DetectedRuntime;

fn project(files: &[(&str, &str)]) -> tempfile::TempDir {
    let dir = tempfile::tempdir().expect("temp dir");
    for (name, body) in files {
        let path = dir.path().join(name);
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).expect("mkdir");
        }
        std::fs::write(path, body).expect("write");
    }
    dir
}

fn names(runtimes: &[DetectedRuntime]) -> Vec<String> {
    runtimes.iter().map(|r| format!("{r:?}")).collect()
}

/// Every lib language is recognised from its own manifest alone.
/// (Mọi ngôn ngữ lib nhận diện chỉ từ manifest của chính nó.)
#[test]
fn every_lib_language_yields_a_runtime() {
    // Label, fixture files, and the runtime the fixture must select.
    // (Nhãn, file fixture, và runtime mà fixture phải chọn.)
    type Case = (
        &'static str,
        &'static [(&'static str, &'static str)],
        &'static str,
    );
    let cases: &[Case] = &[
        (
            "rust",
            &[
                ("Cargo.toml", "[package]\nname=\"x\"\nversion=\"0.1.0\"\n"),
                ("src/lib.rs", ""),
            ],
            "RustLib",
        ),
        ("go", &[("go.mod", "module x\n")], "GoLib"),
        (
            "python",
            &[("pyproject.toml", "[project]\nname=\"x\"\n")],
            "PythonLib",
        ),
        (
            "java-maven",
            &[("pom.xml", "<project></project>")],
            "JvmLib",
        ),
        ("java-gradle", &[("build.gradle", "plugins { }")], "JvmLib"),
        (
            "dotnet",
            &[("A.csproj", "<Project></Project>")],
            "DotNetLib",
        ),
        (
            "swift",
            &[("Package.swift", "// swift-tools-version:5.9\n")],
            "SwiftLib",
        ),
        (
            "typescript",
            &[("package.json", "{\"name\":\"x\",\"version\":\"1.0.0\"}")],
            "TypeScriptLib",
        ),
    ];
    for (label, files, expected) in cases {
        let dir = project(files);
        let detected = names(&detect_lib_runtime(dir.path()));
        assert!(
            detected.iter().any(|name| name == expected),
            "{label}: expected {expected}, got {detected:?}"
        );
    }
}

/// A bare `package.json` is enough — `tsconfig.json` is not required.
///
/// Requiring it excluded TypeScript packages that build with a bundler config
/// and JavaScript libraries, both of which run Node but have no `tsconfig.json`.
/// (`package.json` là đủ — `tsconfig.json` không bắt buộc.)
#[test]
fn a_package_json_without_tsconfig_is_a_typescript_lib() {
    let dir = project(&[("package.json", "{\"name\":\"x\",\"version\":\"1.0.0\"}")]);
    assert!(names(&detect_lib_runtime(dir.path())).contains(&"TypeScriptLib".to_string()));
}

/// A `package.json` naming a web framework is a web app, not a lib.
/// (`package.json` có framework web là app, không phải lib.)
#[test]
fn a_framework_package_json_is_not_a_lib() {
    for framework in ["react", "vue", "svelte", "next", "vite"] {
        let body = format!("{{\"name\":\"x\",\"dependencies\":{{\"{framework}\":\"1\"}}}}");
        let dir = project(&[("package.json", body.as_str())]);
        assert!(
            !names(&detect_lib_runtime(dir.path())).contains(&"TypeScriptLib".to_string()),
            "'{framework}' is a web app, not a lib"
        );
    }
}

/// An empty directory yields no runtime, so `mgc optimizer` says so instead of
/// pretending to work.
/// (Thư mục rỗng không runtime, để `mgc optimizer` báo thay vì giả vờ.)
#[test]
fn an_empty_directory_yields_no_runtime() {
    let dir = project(&[]);
    assert!(detect_lib_runtime(dir.path()).is_empty());
}

// ── Cores with no language-specific runtime ─────────────────────────────
// Godot (`project.godot`), hardware marker projects, and pipeline-only repos
// have no language manifest, yet the machine-level tuning (hardware profile,
// GPU snapshot) applies to them. Refusing with "No runtime detected" left those
// cores with no optimizer at all.
// (Godot, hardware marker, pipeline-only không có manifest ngôn ngữ, nhưng
// tuning mức máy vẫn áp được. Từ chối khiến các core đó không có optimizer.)

use crate::commands::optimizer::runtime_detect::detect_runtimes;

/// A Godot project selects the generic lane instead of reporting nothing.
/// (Project Godot nhận lane chung thay vì báo rỗng.)
#[test]
fn a_godot_project_selects_the_generic_lane() {
    let dir = project(&[("project.godot", "config_version=5\n")]);
    assert!(detect_runtimes(dir.path(), "game").contains(&DetectedRuntime::GenericToolchain));
}

/// A hardware marker project selects the generic lane.
/// (Project hardware marker nhận lane chung.)
#[test]
fn a_hardware_project_selects_the_generic_lane() {
    let dir = project(&[("mgc.toml", "name = \"h\"\necosystem = \"hardware\"\n")]);
    assert!(detect_runtimes(dir.path(), "hardware").contains(&DetectedRuntime::GenericToolchain));
}

/// A Rust game still gets the Rust lane, not the generic one, so a Bevy project
/// keeps its compiler tuning.
/// (Game Rust vẫn nhận lane Rust để giữ tuning compiler.)
#[test]
fn a_rust_game_keeps_the_rust_lane() {
    let dir = project(&[
        ("Cargo.toml", "[package]\nname=\"g\"\nversion=\"0.1.0\"\n"),
        ("src/main.rs", "fn main() {}"),
    ]);
    let runtimes = detect_runtimes(dir.path(), "game");
    assert!(runtimes.contains(&DetectedRuntime::RustLib));
    assert!(!runtimes.contains(&DetectedRuntime::GenericToolchain));
}
