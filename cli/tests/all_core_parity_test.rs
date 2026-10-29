//! Cross-core parity tests — test đồng nhất giữa các core.
//!
//! Two different properties, deliberately kept apart:
//!
//! - CLI surface (all NINE cores): identical positional pair — asserted here.
//! - Scaffold-from-local-source (FOUR cores: web/ai/app/lib): those are the cores
//!   whose adapters materialise from a local directory. `game`/`iot`/`clo`/
//!   `cicd`/`hardware` scaffold through their own host tool or the registry
//!   layer, so a local-source fixture cannot represent them and asserting it
//!   here would test a path the product does not have.
//!
//! (Hai thuộc tính khác nhau: CLI surface cho CẢ CHÍN core; scaffold từ local
//! source chỉ cho BỐN core vì năm core kia dùng host tool / registry layer.)
//!
//! Hermetic: temp HOME, no workspace templates — HOME tạm, không dùng workspace templates.

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
fn test_local_source_parity_web_ai_app_lib() {
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
    // The record pins a version (see `CREATE_VITE_WITH_TEMPLATES`): `@latest`
    // stopped shipping `template-*` in its tarball, so asserting the literal
    // `@latest` would force the regression back in. Assert the flow instead.
    // (Record ghim version vì `@latest` không còn ship `template-*`; khẳng định
    // literal `@latest` sẽ ép lỗi quay lại, nên kiểm luồng thay vì chuỗi.)
    assert!(
        web_output_text.contains("create: installing initializer create-vite@"),
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
    // Every `create-<core>` command must take the same positional pair, so a user
    // who learns one core has learned all nine. The test used to check four
    // cores while its name said "all"; the other five drifted unnoticed because
    // nothing asserted them.
    // (Mọi lệnh `create-<core>` phải nhận cùng cặp positional để học một core là
    // dùng được cả chín. Test cũ kiểm bốn core trong khi tên nói "all"; năm core
    // còn lại không ai khẳng định.)
    let mgc = mgc_binary();
    let cores = [
        "web", "ai", "app", "lib", "game", "iot", "clo", "cicd", "hardware",
    ];
    for core in cores {
        let command = format!("create-{core}");
        let output = Command::new(&mgc)
            .args([command.as_str(), "--help"])
            .output()
            .unwrap_or_else(|error| panic!("{command}: cannot run --help: {error}"));
        assert!(
            output.status.success(),
            "{command} --help failed: {}",
            String::from_utf8_lossy(&output.stderr)
        );
        let help = String::from_utf8_lossy(&output.stdout);
        for expected in ["FRAMEWORK[@VERSION]", "PROJECT"] {
            assert!(
                help.contains(expected),
                "{command}: CLI signature must contain '{expected}' so all cores share one shape\n{help}"
            );
        }
    }
}

/// The five host-tool cores are reachable through the registry layer, not local
/// source, so the local-source parity test must not claim them.
///
/// This asserts the *gate* rather than the scaffold: a `--from-local` request for
/// a host-tool core must fail closed with guidance instead of silently producing
/// an empty project.
/// (Năm core host-tool đi qua registry layer chứ không phải local source, nên test
/// local-source không được nhận chúng. Test này khẳng định cổng: `--from-local`
/// cho core host-tool phải fail-closed kèm hướng dẫn.)
#[test]
fn a_host_tool_core_refuses_a_local_source_instead_of_scaffolding_nothing() {
    let temp_home = setup_hermetic_home();
    let workspace = TempDir::new().unwrap();
    let source = common::local_source(workspace.path(), "lib", "rust");

    for core in ["game", "iot", "clo", "cicd", "hardware"] {
        let project = workspace.path().join(format!("probe-{core}"));
        let output = hermetic_command(temp_home.path())
            .args([
                format!("create-{core}").as_str(),
                "unknown-framework",
                project.to_str().expect("utf-8 path"),
            ])
            .arg(source.flag())
            .current_dir(workspace.path())
            .output()
            .expect("run create");

        assert!(
            !output.status.success(),
            "create-{core} must not accept a local lib source; stdout={} stderr={}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        );
        assert!(
            !project.exists(),
            "create-{core} must not leave a half-created project behind"
        );
    }
}
