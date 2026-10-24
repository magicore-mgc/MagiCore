//! All-core parity test (Phase 2 — user 2026-08-31) — test đồng nhất 4 cores
//! All-core parity test: web/ai/app/lib must create projects equally.
//! Hermetic: temp HOME, no workspace templates/ — hermetic: HOME tạm, không dùng workspace templates/.

#![allow(clippy::unwrap_used)]
#![allow(clippy::needless_borrows_for_generic_args)]

use std::fs;
use std::path::PathBuf;
use std::process::Command;
use tempfile::TempDir;

#[path = "common/mod.rs"]
mod common;

fn mgc_binary() -> PathBuf {
    PathBuf::from(env!("CARGO_BIN_EXE_mgc"))
}

fn setup_hermetic_home() -> TempDir {
    TempDir::new().unwrap()
}

// Scope all environment overrides to the child; parallel tests keep their own home.
// Không đổi environment của test runner; mỗi process có cache/store/template root riêng.
fn hermetic_command(home: &std::path::Path) -> Command {
    let mut command = Command::new(mgc_binary());
    command
        .env("HOME", home)
        .env("USERPROFILE", home)
        .env("MGC_DATA_DIR", home.join("data"))
        .env("MGC_CACHE_DIR", home.join("cache"))
        .env("MGC_TEMPLATES_DIR", home.join("templates"))
        .env("MAGICORE_STORE_ROOT", home.join("store"))
        .env_remove("MAGICORE_TEMPLATE_DIR")
        .env_remove("MGC_NPM_REGISTRY");
    command
}

#[test]
fn test_all_core_parity_online_web_and_native_core_processors() {
    let _temp_home = setup_hermetic_home();
    let temp_workspace = TempDir::new().unwrap();
    let workspace_path = temp_workspace.path();

    // 1. Web vanilla uses the official online creator through MGC dlx.
    let web_output = hermetic_command(_temp_home.path())
        .args(&["create-web", "vanilla", "test-web", "--ts"])
        .current_dir(workspace_path)
        .output()
        .expect("Failed to execute mgc create-web");

    assert!(
        web_output.status.success(),
        "create-web vanilla failed: {}",
        String::from_utf8_lossy(&web_output.stderr)
    );
    let web_output_text = format!(
        "{}{}",
        String::from_utf8_lossy(&web_output.stdout),
        String::from_utf8_lossy(&web_output.stderr)
    );
    assert!(
        web_output_text
            .contains("create: installing initializer create-vite@latest through MagiCore"),
        "web/vanilla did not run the creator through the MagiCore create flow: {web_output_text}"
    );
    let provenance = fs::read_to_string(workspace_path.join("test-web/.mgc-provenance.json"))
        .expect("web generator provenance");
    assert!(provenance.contains("npm:create-vite@"));
    // Canonical provenance schema is `npm:<pkg>@<resolved>#generator=<bin>:<tpl>`
    // (emitted by `run_create_initializer`); it names the exact initializer binary
    // plus template, which the older `#template=` form could not express.
    // Schema provenance chuẩn là `npm:<pkg>@<resolved>#generator=<bin>:<tpl>` (do
    // `run_create_initializer` ghi); nó nêu đúng binary initializer và template,
    // điều mà dạng `#template=` cũ không diễn đạt được.
    assert!(provenance.contains("#generator=vite:vanilla-ts"));
    assert!(!provenance.contains("npm:create-vite@latest"));
    assert!(
        workspace_path.join("test-web/index.html").exists(),
        "web/vanilla: index.html not created"
    );
    assert!(
        workspace_path.join("test-web/.mgc.core").exists(),
        "web: .mgc.core marker not created"
    );

    // 2. AI python-agent reuses an existing python-agent project: the offline
    // branch MagiCore promises when the registry layer cannot supply a layer.
    // AI/python-agent tái dùng project có sẵn: nhánh offline sản phẩm cam kết.
    let ai_source = common::local_source(workspace_path, "ai", "python-agent");
    let ai_output = hermetic_command(_temp_home.path())
        .arg("create-ai")
        .arg("python-agent")
        .arg("test-ai")
        .arg(ai_source.flag())
        .current_dir(workspace_path)
        .output()
        .expect("Failed to execute mgc create-ai");

    assert!(
        ai_output.status.success(),
        "create-ai python-agent failed: {}",
        String::from_utf8_lossy(&ai_output.stderr)
    );
    assert!(
        workspace_path.join("test-ai/pyproject.toml").exists(),
        "ai/python-agent: pyproject.toml not created"
    );
    assert!(
        workspace_path.join("test-ai/.mgc.core").exists(),
        "ai: .mgc.core marker not created"
    );

    // 3. App flutter reuses an existing flutter project.
    // App/flutter tái dùng project có sẵn.
    let app_source = common::local_source(workspace_path, "app", "flutter");
    let app_output = hermetic_command(_temp_home.path())
        .arg("create-app")
        .arg("flutter")
        .arg("test-app")
        .arg(app_source.flag())
        .current_dir(workspace_path)
        .output()
        .expect("Failed to execute mgc create-app");

    assert!(
        app_output.status.success(),
        "create-app flutter failed: {}",
        String::from_utf8_lossy(&app_output.stderr)
    );
    assert!(
        workspace_path.join("test-app/pubspec.yaml").exists(),
        "app/flutter: pubspec.yaml not created"
    );
    assert!(
        workspace_path.join("test-app/.mgc.core").exists(),
        "app: .mgc.core marker not created"
    );

    // 4. Lib rust reuses an existing rust project.
    // Lib/rust tái dùng project có sẵn.
    let lib_source = common::local_source(workspace_path, "lib", "rust");
    let lib_output = hermetic_command(_temp_home.path())
        .arg("create-lib")
        .arg("rust")
        .arg("test-lib")
        .arg(lib_source.flag())
        .current_dir(workspace_path)
        .output()
        .expect("Failed to execute mgc create-lib");

    assert!(
        lib_output.status.success(),
        "create-lib rust failed: {}",
        String::from_utf8_lossy(&lib_output.stderr)
    );
    assert!(
        workspace_path.join("test-lib/Cargo.toml").exists(),
        "lib/rust: Cargo.toml not created"
    );
    assert!(
        workspace_path.join("test-lib/.mgc.core").exists(),
        "lib: .mgc.core marker not created"
    );

    // Verify all core markers
    let cores = ["web", "ai", "app", "lib"];
    for core in &cores {
        let marker_path = workspace_path.join(format!("test-{core}/.mgc.core"));
        let content = fs::read_to_string(marker_path).expect("Failed to read .mgc.core");
        assert_eq!(content.trim(), *core, "{core}: .mgc.core content mismatch");
    }
}

#[test]
fn test_all_core_cli_surface_uniform() {
    // Verify all cores have uniform CLI signature: <framework[@version]> <project>
    // (not testing execution, only clap definition)

    let mgc = mgc_binary();

    // Web: mgc create-web <FRAMEWORK[@VERSION]> <PROJECT>
    let web_help = Command::new(&mgc)
        .args(&["create-web", "--help"])
        .output()
        .expect("Failed to get create-web help");
    let web_help_str = String::from_utf8_lossy(&web_help.stdout);
    assert!(
        web_help_str.contains("FRAMEWORK[@VERSION]") && web_help_str.contains("PROJECT"),
        "create-web: CLI signature mismatch"
    );

    // AI: mgc create-ai <FRAMEWORK[@VERSION]> <PROJECT>
    let ai_help = Command::new(&mgc)
        .args(&["create-ai", "--help"])
        .output()
        .expect("Failed to get create-ai help");
    let ai_help_str = String::from_utf8_lossy(&ai_help.stdout);
    assert!(
        ai_help_str.contains("FRAMEWORK[@VERSION]") && ai_help_str.contains("PROJECT"),
        "create-ai: CLI signature mismatch"
    );

    // App: mgc create-app <FRAMEWORK[@VERSION]> <PROJECT>
    let app_help = Command::new(&mgc)
        .args(&["create-app", "--help"])
        .output()
        .expect("Failed to get create-app help");
    let app_help_str = String::from_utf8_lossy(&app_help.stdout);
    assert!(
        app_help_str.contains("FRAMEWORK[@VERSION]") && app_help_str.contains("PROJECT"),
        "create-app: CLI signature mismatch"
    );

    // Lib: mgc create-lib <FRAMEWORK[@VERSION]> <PROJECT> (Phase 2 fix — was just <PROJECT>)
    let lib_help = Command::new(&mgc)
        .args(&["create-lib", "--help"])
        .output()
        .expect("Failed to get create-lib help");
    let lib_help_str = String::from_utf8_lossy(&lib_help.stdout);
    assert!(
        lib_help_str.contains("FRAMEWORK[@VERSION]") && lib_help_str.contains("PROJECT"),
        "create-lib: CLI signature mismatch (should be <FRAMEWORK[@VERSION]> <PROJECT>)"
    );
}
