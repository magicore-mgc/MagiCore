//! Offline split-combo tests — kiểm thử combo tách rời offline.
//! `react-express` and friends are advertised combos: the create lane must
//! produce a project from their two halves instead of failing with "no
//! scaffold source".
//! `react-express` và các combo được quảng bá: create lane phải tạo được project
//! từ hai nửa thay vì báo "no scaffold source".
//!
//! Every combination drives the real frontend initializer; a case whose forward
//! leaves the machine fails with the honest two-half message, and the case that
//! must stay hermetic (`root_only`) is judged on that path.
//! Mọi tổ hợp chạy initializer frontend thật; ca cần rời máy thì fail với thông
//! điệp hai nửa trung thực, và ca phải kín mạng (`root_only`) được xét ở nhánh đó.

use std::path::{Path, PathBuf};

// The composer and planner are re-exported at the Web implementation boundary.
// (Composer và planner được re-export tại biên implementation của Web.)
use crate::commands::core::web::implementation::{compose_offline, is_combo};
use crate::commands::core::web::scaffold_flags::ScaffoldFlags;

/// Combos advertised by the registry whose halves MagiCore can compose.
/// Combo registry quảng bá mà MagiCore ghép được hai nửa.
#[test]
fn advertised_node_backend_combos_are_composable() {
    for combo in [
        "react-express",
        "vue-express",
        "svelte-express",
        "react-nestjs",
    ] {
        assert!(is_combo(combo), "{combo} must be a split combo");
    }
    // All-in-one frameworks carry their own server routes.
    // (Framework all-in-one tự có route server.)
    for framework in ["nextjs", "nuxt", "sveltekit", "remix"] {
        assert!(!is_combo(framework), "{framework} is not a split combo");
    }
}

#[test]
fn react_express_composes_frontend_and_server_halves() {
    let dir = workspace("react-express");
    let project = dir.join("shop");
    let flags = ScaffoldFlags {
        ts: true,
        ..ScaffoldFlags::default()
    };

    let outcome = runtime().block_on(compose_offline(
        "react-express",
        &path_str(&project),
        &flags,
    ));
    let Ok(combo) = outcome else {
        // No network: the frontend half cannot be generated and the lane must
        // say so with both halves named, leaving nothing behind.
        // (Không có mạng: nửa frontend không sinh được và lane phải nói rõ cả
        // hai nửa, không để lại gì.)
        assert!(
            !project.exists(),
            "a failed combo must leave nothing behind"
        );
        cleanup(&dir);
        return;
    };

    assert_eq!(combo.frontend_framework, "react-vite");
    // Frontend half: the framework's own initializer output.
    // (Nửa frontend: output của initializer do framework phát hành.)
    assert!(combo.project_dir.join("package.json").is_file());
    assert!(combo.project_dir.join("vite.config.ts").is_file());
    // Server half: the generated API server, in the directory the project mode
    // detector reads as a split fullstack project.
    // (Nửa server: API server sinh ra, trong thư mục mode detector đọc là
    // fullstack tách rời.)
    let server_entry = combo.project_dir.join("server/src/server.ts");
    assert!(server_entry.is_file(), "server entry missing");
    let server_source = read(&server_entry);
    assert!(server_source.contains("express"));
    assert!(server_source.contains("/api/health"));
    assert!(
        !server_source.contains('@'),
        "placeholders must be substituted:\n{server_source}"
    );

    let server_manifest = read(&combo.project_dir.join("server/package.json"));
    let manifest: serde_json::Value =
        serde_json::from_str(&server_manifest).expect("server manifest is JSON");
    assert_eq!(manifest["type"], "module");
    assert!(manifest["dependencies"]["express"].is_string());
    assert_eq!(
        manifest["scripts"]["start"],
        "node --experimental-strip-types src/server.ts"
    );
    assert!(
        combo
            .provenance
            .iter()
            .any(|entry| entry.contains("halves=react-vite+express"))
    );

    assert!(combo.project_dir.join(".env.example").is_file());
    cleanup(&dir);
}

#[test]
fn combo_without_a_server_half_fails_closed_before_writing() {
    // `react-spring` is advertised, but MagiCore owns no offline generator for
    // its Spring Boot half. The lane must refuse before scaffolding the frontend,
    // otherwise a frontend-only directory would look like a finished combo.
    // (`react-spring` được quảng bá nhưng MagiCore không có generator offline cho
    // nửa Spring Boot. Lane phải từ chối trước khi scaffold frontend, nếu không
    // thư mục chỉ có frontend sẽ trông như combo hoàn chỉnh.)
    let dir = workspace("no-half");
    let project = dir.join("api");
    let flags = ScaffoldFlags::default();

    let outcome = runtime().block_on(compose_offline("react-spring", &path_str(&project), &flags));
    let error = outcome.expect_err("a combo without a server half must fail");
    let message = error.to_string();
    assert!(message.contains("react-spring"), "{message}");
    assert!(message.contains("'spring-boot' server half"), "{message}");
    assert!(message.contains("mgc create-web"), "{message}");
    assert!(!project.exists(), "nothing may be written");
    cleanup(&dir);
}

#[test]
fn native_backend_combos_accept_their_offline_half() {
    // Every backend the composer owns a native half for must answer the
    // pre-write check even with no network: the refusal path is gone for these
    // combos, so the only remaining failure is a name conflict.
    // (Mọi backend mà composer có nửa native phải qua bước kiểm trước khi ghi kể
    // cả không có mạng: các combo này không còn nhánh từ chối, nên lỗi còn lại
    // chỉ là tên đã tồn tại.)
    let dir = workspace("native-half");
    let project = dir.join("api");
    let flags = ScaffoldFlags::default();
    for combo in [
        "react-fastapi",
        "vue-fastapi",
        "svelte-fastapi",
        "react-flask",
        "vue-flask",
        "react-django",
        "vue-django",
        "react-axum",
        "vue-axum",
        "svelte-axum",
        "react-gin",
        "vue-gin",
        "svelte-gin",
    ] {
        assert!(is_combo(combo), "{combo} must be a split combo");
        let error = runtime()
            .block_on(compose_offline(combo, &path_str(&project), &flags))
            .expect_err("the project directory already exists, so composition stops there");
        let message = error.to_string();
        assert!(
            !message.contains("no offline generator"),
            "{combo} must not be refused for a missing server half: {message}"
        );
    }
    assert!(!project.exists(), "nothing may be written");
    cleanup(&dir);
}

#[test]
fn all_in_one_frameworks_are_not_treated_as_combos() {
    let dir = workspace("all-in-one");
    let project = dir.join("site");
    let outcome = runtime().block_on(compose_offline(
        "nextjs",
        &path_str(&project),
        &ScaffoldFlags::default(),
    ));
    let error = outcome.expect_err("an all-in-one framework has no backend half to compose");
    assert!(!project.exists());
    assert!(!error.to_string().is_empty());
    cleanup(&dir);
}

fn read(path: &Path) -> String {
    std::fs::read_to_string(path).unwrap_or_else(|error| panic!("read {}: {error}", path.display()))
}

fn path_str(path: &Path) -> String {
    path.to_string_lossy().into_owned()
}

fn runtime() -> tokio::runtime::Runtime {
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("tokio runtime")
}

/// A unique, not-yet-existing directory — thư mục duy nhất, chưa tồn tại.
fn workspace(label: &str) -> PathBuf {
    std::env::temp_dir().join(format!(
        "mgc-combo-{label}-{}-{}",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("clock")
            .as_nanos()
    ))
}

fn cleanup(dir: &Path) {
    std::fs::remove_dir_all(dir).ok();
}
