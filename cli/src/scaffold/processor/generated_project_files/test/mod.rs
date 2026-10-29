//! Generated project file tests — test file sinh cho project.
//! The `.gitignore` template decides what a NEW project tracks. A machine-local
//! artifact that reaches git pollutes every clone, so each one is asserted by
//! the operation that writes it.
//! (Template `.gitignore` quyết định project MỚI track gì. Artefact riêng của máy
//! lọt vào git sẽ làm bẩn mọi clone, nên mỗi cái được khẳng định theo thao tác ghi
//! nó.)

use super::common_gitignore;

/// The rows a scaffolded `.gitignore` must carry, each paired with the operation
/// that writes the path.
/// (Các dòng `.gitignore` phải có, mỗi dòng gắn với thao tác ghi nó.)
const REQUIRED_IGNORES: &[(&str, &str)] = &[
    (
        ".mgc-optimizer/",
        "mgc optimizer (machine profile + gpu.env)",
    ),
    (
        ".magicore/tmp-optimizer/",
        "game optimizer staging directory",
    ),
    (".magicore/cache", "MagiCore cache"),
    (".magicore/tmp", "MagiCore temp"),
    (".env", "local environment"),
];

#[test]
fn every_machine_local_path_is_ignored_for_every_core() {
    // Every core scaffolds the same base template; a per-core omission would only
    // show up when that core writes the path.
    // (Mọi core dùng cùng template gốc; thiếu theo core chỉ lộ khi core đó ghi.)
    let cores = [
        "web", "ai", "app", "lib", "game", "iot", "clo", "cicd", "hardware",
    ];
    for core in cores {
        // A representative framework per core; the base set is core-independent
        // and this asserts exactly that.
        // (Framework đại diện mỗi core; base set không phụ thuộc core.)
        let body = common_gitignore(core, "probe");
        for (path, writer) in REQUIRED_IGNORES {
            assert!(
                body.lines().any(|line| line == *path),
                "{core}: '{path}' ({writer}) is missing from .gitignore, so a fresh \
                 scaffold would track it"
            );
        }
    }
}

/// A core-specific build directory is still ignored for that core.
/// (Thư mục build riêng của core vẫn được bỏ qua.)
/// Each toolchain directory a core's own code skips must also be git-ignored.
///
/// Two drifts were real: `ai` ignored `.venv` but not `venv` (its walker skips
/// both), and `app` ignored `.dart_tool`/`build` but not `.flutter-plugins*` or
/// `Pods` (its container ignore list does). Both directions are asserted here.
/// (Hai chỗ lệch từng có thật: `ai` ignore `.venv` mà không `venv`; `app` ignore
/// `.dart_tool`/`build` mà không `.flutter-plugins*`/`Pods`.)
#[test]
fn every_core_toolchain_directory_is_ignored() {
    let cases: &[(&str, &str, &[&str])] = &[
        (
            "web",
            "react",
            &["node_modules", "dist", ".next", ".svelte-kit"],
        ),
        ("lib", "rust", &["target", "dist", "__pycache__"]),
        ("lib", "python", &[".venv", "venv", ".pytest_cache"]),
        (
            "ai",
            "python",
            &[".venv", "venv", "__pycache__", ".pytest_cache"],
        ),
        (
            "clo",
            "terraform",
            &[".pulumi", ".terraform", ".terraform.lock.hcl"],
        ),
        ("cicd", "github-actions", &[".artifacts", ".secrets"]),
        ("iot", "platformio", &[".pio", "target", "build"]),
        (
            "app",
            "flutter",
            &[
                ".dart_tool",
                "build",
                ".gradle",
                ".build",
                ".flutter-plugins",
                ".flutter-plugins-dependencies",
                "Pods",
            ],
        ),
        ("game", "godot", &[".godot", ".import"]),
        ("game", "unity", &["Library", "Temp", "Build"]),
        ("game", "bevy", &["target"]),
    ];
    for (core, framework, expected) in cases {
        let body = common_gitignore(core, framework);
        for path in *expected {
            assert!(
                body.lines().any(|line| line == *path),
                "{core}/{framework}: '{path}' is skipped by this core's own code but \
                 missing from .gitignore"
            );
        }
    }
}

/// The template ends with exactly one newline so concatenating it into another
/// file cannot splice two lines together.
/// (Template kết thúc đúng một newline để nối vào file khác không dính dòng.)
#[test]
fn the_template_ends_with_a_single_newline() {
    let body = common_gitignore("web", "react");
    assert!(body.ends_with('\n'));
    assert!(!body.ends_with("\n\n"));
}

// ── Merge into a framework's .gitignore ─────────────────────────────────
// A Web project arrives with `create-vite`/`create-next-app`'s own
// `.gitignore`. Replacing it would drop the framework's rules (including `!`
// negations); skipping it left MagiCore's entries absent, so the project tracked
// `.mgc-optimizer/` and `.magicore/store`. The merge is the only shape that
// keeps both.
// (Project Web có `.gitignore` riêng. Ghi đè mất rule framework (kể cả `!`
// negation); bỏ qua thì mất entry MagiCore, project track `.mgc-optimizer/`.
// Hợp nhất là dạng duy nhất giữ cả hai.)

use crate::scaffold::processor::Scaffolder;

#[test]
fn merging_preserves_framework_lines_and_appends_missing_entries() {
    let dir = tempfile::tempdir().expect("temp dir");
    let path = dir.path().join(".gitignore");
    std::fs::write(
        &path,
        "# Logs\nlogs\nnode_modules\ndist-ssr\n.vscode/*\n!.vscode/extensions.json\n",
    )
    .expect("write existing");

    let generated = common_gitignore("web", "react-vite");
    let merged = Scaffolder::merge_gitignore_for_test(&path, &generated).expect("merge");

    // Framework lines survive verbatim, including the negation.
    // (Dòng framework còn nguyên, kể cả negation.)
    for kept in [
        "# Logs",
        "logs",
        "node_modules",
        "dist-ssr",
        ".vscode/*",
        "!.vscode/extensions.json",
    ] {
        assert!(merged.lines().any(|line| line == kept), "lost '{kept}'");
    }
    // MagiCore entries are present exactly once.
    // (Entry MagiCore có đúng một lần.)
    for added in [".mgc-optimizer/", ".magicore/store", ".magicore/exec.log"] {
        assert_eq!(
            merged.lines().filter(|line| *line == added).count(),
            1,
            "'{added}' must appear exactly once"
        );
    }
}

#[test]
fn merging_an_absent_file_writes_the_full_list() {
    let dir = tempfile::tempdir().expect("temp dir");
    let path = dir.path().join(".gitignore");
    let generated = common_gitignore("web", "react-vite");
    let merged = Scaffolder::merge_gitignore_for_test(&path, &generated).expect("merge");
    assert_eq!(merged, generated);
}

#[test]
fn merging_twice_is_idempotent() {
    let dir = tempfile::tempdir().expect("temp dir");
    let path = dir.path().join(".gitignore");
    std::fs::write(&path, "logs\nnode_modules\n").expect("write");
    let generated = common_gitignore("web", "react-vite");

    let once = Scaffolder::merge_gitignore_for_test(&path, &generated).expect("first");
    std::fs::write(&path, &once).expect("persist");
    let twice = Scaffolder::merge_gitignore_for_test(&path, &generated).expect("second");
    assert_eq!(once, twice, "a second merge must not duplicate entries");
}

/// A Web project must keep the framework's README, not MagiCore's.
///
/// `write_common_files` runs BEFORE `write_core_files`, so checking
/// `README.md.exists()` there could never be true: the framework's file did not
/// exist yet. MagiCore wrote its own README and the framework then overwrote it —
/// wasted work, and the guard gave a false impression of protecting the file.
/// (write_common_files chạy TRƯỚC write_core_files nên `exists()` không bao giờ
/// đúng; MagiCore ghi README rồi framework đè.)
#[test]
fn the_common_readme_names_the_project_and_core() {
    let body = super::super::common_readme("demo", "web", "react-vite", &[]);
    assert!(
        body.contains("demo"),
        "the README must name the project, otherwise it is unusable as documentation"
    );
    assert!(
        body.to_lowercase().contains("web"),
        "the README must name the core"
    );
}

/// The core-specific README differs per core, so it is not a single blob.
/// (README theo core khác nhau, không phải một khối chung.)
#[test]
fn each_core_readme_mentions_its_own_core() {
    for core in ["web", "lib", "ai", "app", "game", "iot", "clo", "cicd"] {
        let body = super::super::common_readme("demo", core, "probe", &[]);
        assert!(
            body.to_lowercase().contains(core),
            "{core}: its README must name the core it documents"
        );
    }
}
