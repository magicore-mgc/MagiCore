//! Static framework directory: which core/framework pairs MagiCore knows.
//! Danh mục framework tĩnh: cặp core/framework mà MagiCore biết.
//!
//! This is data only — names, cores and modes. It never contains framework
//! source: project files always come from the registry layer or from a
//! project already present on the user's machine.
//! Đây chỉ là dữ liệu — tên, core và mode. Không bao giờ chứa source
//! framework: file dự án luôn đến từ registry layer hoặc từ project đã có
//! trên máy user.

/// Framework ids accepted per core.
/// Id framework được chấp nhận theo core.
pub const GAME: &[&str] = &["unity", "godot", "unreal", "bevy"];
pub const CLOUD: &[&str] = &[
    "terraform",
    "terraform-gcp",
    "cdk",
    "cdk-typescript",
    "cloudflare",
    "lambda",
    "pulumi",
];
/// CI/CD providers `ci-generate` can really emit a pipeline for.
///
/// The list mirrors the emitter, not the detector: `aws`, `gcp` and `cloudflare`
/// are detectable, but no pipeline template exists for them, so accepting the
/// name here would offer a `create-cicd` that cannot produce a project.
/// (Danh sách khớp với emitter, không khớp với detector: `aws`, `gcp`,
/// `cloudflare` nhận diện được nhưng chưa có template pipeline, nên nhận tên ở
/// đây sẽ mời gọi một `create-cicd` không sinh được project.)
pub const CICD: &[&str] = &[
    "github-actions",
    "gitlab",
    "circleci",
    "jenkins",
    "azure-pipelines",
    "bitbucket-pipelines",
    "argocd",
];
pub const IOT: &[&str] = &[
    "platformio",
    "firmware",
    "zephyr",
    "zephyr-arm",
    "esp32-rust",
    "esp32",
    "esp32c3",
    "esp32s3",
    "esp32dev",
    "nodemcu-32s",
    "nrf52dk_nrf52832",
    "stm32f4_disc",
];
pub const APP: &[&str] = &[
    "kotlin",
    "swift",
    "maui",
    "objc",
    "tauri",
    "react-native",
    "flutter",
    "multi",
];
pub const LIB: &[&str] = &["rust", "ts", "typescript", "python", "go", "dotnet", "java"];
/// AI scaffold frameworks — the two shapes `AiFramework` actually models.
///
/// `pytorch`, `tensorflow`, `jax`, `onnx` and `candle` are *runtime* signatures
/// (project detection and optimizer hints), not scaffolds: `mgc create-ai
/// pytorch` never existed, and listing them here made the registry claim six
/// frameworks while only two could be created.
/// (Framework scaffold AI — đúng hai dạng `AiFramework` mô hình hoá. pytorch/
/// tensorflow/jax/onnx/candle là chữ ký runtime, không phải scaffold; liệt kê
/// chúng khiến registry claim 6 framework trong khi chỉ tạo được 2.)
pub const AI: &[&str] = &["mcp-server", "python-agent"];

/// AI runtimes a project may declare — detection only, never a scaffold target.
/// Runtime AI project có thể khai — chỉ để detect, không phải đích scaffold.
pub const AI_RUNTIMES: &[&str] = &["candle", "jax", "onnx", "pytorch", "tensorflow"];
pub const HARDWARE: &[&str] = &["optimizer", "bench"];
/// Web backend ids with a real scaffold source, so `is_supported` may say yes.
///
/// An id belongs here only when `mgc create-web <id>` can resolve a project
/// from some source the create lane really has: an initializer catalog record
/// (`commands/core/web/create/initializers/catalog`), or an official/local
/// project the lane can materialize for that framework. The remaining known
/// backends live in `WEB_BACKEND_UNSUPPORTED`.
/// (Id backend Web CÓ nguồn scaffold thật để `is_supported` được phép nói có.
/// Một id chỉ ở đây khi `mgc create-web <id>` resolve được project từ nguồn mà
/// create lane thật sự có: record initializer, hoặc project chính thức/local
/// mà lane materialize được. Các backend đã biết còn lại ở
/// `WEB_BACKEND_UNSUPPORTED`.)
pub const WEB_BACKEND: &[&str] = &[
    "actix-web",
    "axum",
    "django",
    "dotnet-minimal",
    "echo",
    "express",
    "fastapi",
    "fiber",
    "flask",
    "gin",
    "laravel",
    "nest",
    "nestjs",
    "quarkus",
    "spring-boot",
    "symfony",
];

/// Web backend ids MagiCore knows but cannot scaffold yet — no initializer
/// record, no framework metadata, and no verified scaffold source.
///
/// These were previously listed in `WEB_BACKEND`, so `is_supported` returned
/// true while `mgc create-web fastify x` failed closed with "no scaffold
/// source" (or, for `koa`/`spring`, with a bare "Unsupported web framework").
/// They stay named here on purpose: `create-web` reports "not yet supported for
/// scaffolding" instead of a generic error, and a framework graduates by
/// gaining a catalog record or a verified source, not by editing this list.
/// (Id backend Web MagiCore biết nhưng chưa scaffold được — không có record,
/// không có metadata framework, không nguồn đã xác minh. Trước đây chúng nằm
/// trong `WEB_BACKEND` nên `is_supported` trả true trong khi `create-web
/// fastify x` fail-closed. Chúng được nêu tên ở đây để `create-web` báo "not
/// yet supported for scaffolding" thay vì lỗi chung.)
pub const WEB_BACKEND_UNSUPPORTED: &[&str] = &["fastify", "hono", "koa", "spring", "trpc"];

/// Aliases `normalize_web_framework` maps onto a canonical id.
///
/// A gate that ran *after* normalisation would accept these, but it runs at the
/// CLI boundary on the user's literal word — so the registry must list the
/// aliases too, or `create-web ts` is rejected while the product supports it.
/// Every alias target here is a scaffoldable id, so no alias needs a separate
/// rejected list.
/// (Alias mà `normalize_web_framework` quy về id chuẩn. Cổng chạy tại CLI
/// boundary trên đúng chữ người dùng gõ, nên registry phải liệt kê cả alias,
/// nếu không `create-web ts` bị từ chối trong khi sản phẩm vẫn hỗ trợ. Mọi đích
/// alias ở đây đều scaffold được nên không cần danh sách alias bị từ chối.)
pub const WEB_ALIAS: &[&str] = &[
    "backend",
    "custom",
    "dotnet",
    "frontend",
    "fullstack",
    "go",
    "java",
    "next-app",
    "node",
    "php",
    "python",
    "rust",
    "ts",
];

/// Backend ids that only exist as half of a fullstack combination.
/// Id backend chỉ tồn tại trong cặp fullstack.
pub const WEB_FULLSTACK: &[&str] = &[
    "react-actix-web",
    "react-axum",
    "react-django",
    "react-echo",
    "react-express",
    "react-fastapi",
    "react-fiber",
    "react-flask",
    "react-gin",
    "react-nestjs",
    "react-quarkus",
    "react-symfony",
    "svelte-axum",
    "svelte-express",
    "svelte-fastapi",
    "svelte-gin",
    "svelte-quarkus",
    "vue-actix-web",
    "vue-axum",
    "vue-django",
    "vue-echo",
    "vue-express",
    "vue-fastapi",
    "vue-fiber",
    "vue-flask",
    "vue-gin",
    "vue-laravel",
    "vue-nestjs",
    "vue-quarkus",
    "vue-symfony",
];
pub const WEB_FRONTEND: &[&str] = &[
    "angular",
    "astro",
    "docusaurus",
    "lit",
    "next",
    "nextjs",
    "nuxt",
    "preact",
    "qwik",
    "react",
    "react-vite",
    "remix",
    "solid",
    "solidjs",
    "svelte",
    "sveltekit",
    "tauri",
    "turbo",
    "vanilla",
    "vite",
    "vitepress",
    "vue",
    "vue-cli",
    "vue-vite",
];

/// Board id to framework mapping for IoT.
/// Map board id sang framework cho IoT.
pub fn iot_framework_for_board(board: &str) -> Option<&'static str> {
    match board {
        "esp32" | "esp32c3" | "esp32s3" => Some("esp32-rust"),
        "esp32dev" | "nodemcu-32s" => Some("platformio"),
        "nrf52dk_nrf52832" | "stm32f4_disc" => Some("zephyr-arm"),
        _ => None,
    }
}

/// Default board for an IoT framework, when one is registered.
/// Board mặc định cho framework IoT khi đã đăng ký.
pub fn iot_default_board(framework: &str) -> Option<&'static str> {
    match framework {
        "esp32-rust" => Some("esp32c3"),
        "platformio" | "firmware" => Some("esp32dev"),
        "zephyr" | "zephyr-arm" => Some("nrf52dk_nrf52832"),
        _ => None,
    }
}

/// Local-project ecosystem for a CLI core id (`clo` is stored as `cloud`).
///
/// This is the LAYER/REGISTRY namespace, never the project identity: `cloud`
/// is the published template directory and the npm package namespace, while
/// the CLI identity stays `clo` (see `Ecosystem::passthrough_namespace`).
/// An unknown name borrows the caller's string so the failure names it.
/// (Ecosystem project local cho core id CLI — namespace layer/registry, KHÔNG
/// phải identity project. Tên lạ mượn chuỗi của caller để lỗi nêu đúng tên.)
pub fn ecosystem_for_core(core: &str) -> &str {
    match mgc_types::Ecosystem::from_str(core) {
        Some(ecosystem) => ecosystem.passthrough_namespace(mgc_types::NamespaceRole::TemplateLayer),
        None => core,
    }
}

/// npm registry package namespace for the template layer of `core`.
/// Namespace package trên npm registry cho layer template của `core`.
pub fn template_registry_namespace(core: &str) -> &str {
    ecosystem_for_core(core)
}

/// Framework marker file that proves a local project really is this
/// framework. `None` means the ecosystem has no optional marker: `mgc.toml`
/// identity alone is enough.
/// File marker framework chứng minh project local đúng framework này. `None`
/// nghĩa là ecosystem không có marker tùy chọn: chỉ danh tính `mgc.toml` là đủ.
pub fn framework_marker(ecosystem: &str, framework: &str) -> Option<&'static [&'static str]> {
    match (ecosystem, framework) {
        ("web", "django") => Some(&["manage.py"]),
        ("web", "symfony") => Some(&["bin", "console"]),
        ("web", "flask") | ("web", "fastapi") => Some(&["pyproject.toml"]),
        ("app", "flutter") => Some(&["pubspec.yaml"]),
        ("lib", "rust") => Some(&["Cargo.toml"]),
        ("lib", "python") => Some(&["pyproject.toml"]),
        _ => None,
    }
}

/// Detect framework from markers in a directory path.
///
/// Checks for framework-specific marker files to identify the framework type.
/// Returns `None` if no markers match or if the directory doesn't exist.
/// (Nhận diện framework từ marker file trong thư mục. Trả `None` nếu không có
/// marker khớp hoặc thư mục không tồn tại.)
pub fn detect_framework_from_markers(
    ecosystem: &str,
    dir: &std::path::Path,
) -> Option<&'static str> {
    let frameworks = match ecosystem {
        "web" => WEB_BACKEND,
        "app" => APP,
        "lib" => LIB,
        _ => return None,
    };

    for framework in frameworks {
        if let Some(markers) = framework_marker(ecosystem, framework) {
            let all_exist = markers.iter().all(|marker| {
                let path = dir.join(marker);
                path.exists()
            });
            if all_exist {
                return Some(framework);
            }
        }
    }
    None
}

/// Validate that a slice of framework ids has no duplicates.
///
/// Returns `Ok(())` if all ids are unique, or `Err(duplicate)` with the first
/// duplicate found. Used in tests and at build time to catch registry bugs.
/// (Validate slice id framework không có trùng lặp. Trả `Ok` nếu unique, `Err`
/// với duplicate đầu tiên. Dùng trong test và build-time catch lỗi registry.)
pub fn validate_no_duplicates<'a>(ids: &'a [&'a str]) -> Result<(), &'a str> {
    let mut seen = std::collections::BTreeSet::new();
    for id in ids {
        if !seen.insert(*id) {
            return Err(id);
        }
    }
    Ok(())
}

/// Validate that all framework IDs across all cores are globally unique where
/// they should be.
///
/// Web frameworks are allowed to appear in multiple families (frontend, backend,
/// fullstack) but a framework should not leak between cores. Returns the first
/// cross-core collision found, or `Ok(())` if validation passes.
/// (Validate toàn bộ id framework unique toàn cục nếu cần. Web cho phép xuất
/// hiện nhiều nhóm nhưng không leak giữa cores. Trả collision đầu hoặc `Ok`.)
pub fn validate_all_ids_unique() -> Result<(), String> {
    // Check within-array duplicates first.
    for (name, ids) in [
        ("GAME", GAME),
        ("CLOUD", CLOUD),
        ("CICD", CICD),
        ("IOT", IOT),
        ("APP", APP),
        ("LIB", LIB),
        ("AI", AI),
        ("AI_RUNTIMES", AI_RUNTIMES),
        ("HARDWARE", HARDWARE),
        ("WEB_BACKEND", WEB_BACKEND),
        ("WEB_FRONTEND", WEB_FRONTEND),
        ("WEB_FULLSTACK", WEB_FULLSTACK),
        ("WEB_ALIAS", WEB_ALIAS),
        ("WEB_BACKEND_UNSUPPORTED", WEB_BACKEND_UNSUPPORTED),
    ] {
        if let Err(dup) = validate_no_duplicates(ids) {
            return Err(format!("Duplicate id '{dup}' in {name}"));
        }
    }

    // Check cross-core leaks (exclude Web which legitimately spans families).
    let mut global = std::collections::BTreeMap::new();
    for (core, ids) in [
        ("game", GAME),
        ("cloud", CLOUD),
        ("cicd", CICD),
        ("iot", IOT),
        ("app", APP),
        ("lib", LIB),
        ("ai", AI),
        ("hardware", HARDWARE),
    ] {
        for id in ids {
            if let Some(first_core) = global.insert(*id, core) {
                return Err(format!(
                    "Id '{id}' appears in both '{first_core}' and '{core}'"
                ));
            }
        }
    }

    Ok(())
}

/// Whether `core`/`framework` is a known AND scaffoldable pair.
///
/// Returns false for an id in `WEB_BACKEND_UNSUPPORTED`: MagiCore knows the
/// name but cannot scaffold it, and `is_known_web_unsupported` lets the caller
/// say so honestly.
/// (Cặp `core`/`framework` vừa được biết vừa scaffold được. Trả false cho id
/// trong `WEB_BACKEND_UNSUPPORTED`: MagiCore biết tên nhưng chưa scaffold được,
/// và `is_known_web_unsupported` để caller báo đúng sự thật.)
pub fn is_supported(core: &str, framework: &str) -> bool {
    let table = match core {
        "game" => GAME,
        "clo" | "cloud" => CLOUD,
        "cicd" => CICD,
        "iot" => IOT,
        "app" => APP,
        "lib" => LIB,
        "ai" => AI,
        "hardware" => HARDWARE,
        "web" => {
            return WEB_BACKEND.contains(&framework)
                || WEB_FRONTEND.contains(&framework)
                || WEB_FULLSTACK.contains(&framework)
                || WEB_ALIAS.contains(&framework);
        }
        _ => return false,
    };
    table.contains(&framework)
}

/// Whether `framework` is a Web id MagiCore knows but cannot scaffold yet.
///
/// `create-web` uses this to tell "known but not scaffoldable yet" apart from
/// "not a Web framework at all", so a user who types `fastify` or `koa` hears
/// the honest roadmap message instead of a generic unknown-framework error.
/// (Id Web MagiCore biết nhưng chưa scaffold được. `create-web` dùng để phân
/// biệt "đã biết nhưng chưa scaffold được" với "không phải framework Web".)
pub fn is_known_web_unsupported(framework: &str) -> bool {
    WEB_BACKEND_UNSUPPORTED.contains(&framework)
}

/// Every framework id the registry accepts for `core`, in stable order.
///
/// Used by `create-*` to fail closed with an actionable list instead of
/// scaffolding metadata for a name nobody can build.
/// (Toàn bộ id registry chấp nhận cho `core`, thứ tự ổn định — để `create-*`
/// fail-closed kèm danh sách thay vì scaffold metadata cho tên không build được.)
pub fn supported_frameworks(core: &str) -> Vec<&'static str> {
    match core {
        "game" => GAME.to_vec(),
        "clo" | "cloud" => CLOUD.to_vec(),
        "cicd" => CICD.to_vec(),
        "iot" => IOT.to_vec(),
        "app" => APP.to_vec(),
        "lib" | "library" => LIB.to_vec(),
        "ai" => AI.to_vec(),
        "hardware" => HARDWARE.to_vec(),
        "web" => web_all_ids(),
        _ => vec![],
    }
}

/// Web ids from all accepted families, concatenated for diagnostics.
///
/// Only ids `is_supported("web", ..)` accepts appear here, so the "Supported
/// 'web' frameworks" list printed on a rejection never re-advertises a name
/// that cannot be scaffolded.
///
/// **This is generated dynamically** from WEB_FRONTEND + WEB_BACKEND +
/// WEB_FULLSTACK + WEB_ALIAS to prevent sync issues. Use `web_all_ids()` at
/// runtime.
/// (Id Web của mọi nhóm được nhận, nối lại để báo lỗi. **Generated động** từ
/// các bảng thành phần để tránh lỗi sync. Dùng `web_all_ids()` runtime.)
#[deprecated(
    since = "1.2.0",
    note = "Use web_all_ids() instead - WEB_ALL is now dynamically generated"
)]
pub const WEB_ALL: &[&str] = &[];

/// Returns all accepted Web framework ids (frontend, backend, fullstack, aliases).
///
/// Generated dynamically from component arrays to prevent sync issues. This is
/// the source of truth for "Supported 'web' frameworks" diagnostics.
/// (Trả toàn bộ id Web được nhận — generated động để tránh lỗi sync.)
pub fn web_all_ids() -> Vec<&'static str> {
    let mut ids = Vec::with_capacity(
        WEB_FRONTEND.len() + WEB_BACKEND.len() + WEB_FULLSTACK.len() + WEB_ALIAS.len(),
    );
    ids.extend_from_slice(WEB_FRONTEND);
    ids.extend_from_slice(WEB_BACKEND);
    ids.extend_from_slice(WEB_FULLSTACK);
    ids.extend_from_slice(WEB_ALIAS);
    ids
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
