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
pub const CICD: &[&str] = &["argocd", "aws", "cloudflare", "gcp", "github-actions"];
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
/// Web backend/frontend ids known to MagiCore (source still comes from the
/// registry layer; this list only answers "is this framework supported?").
/// Id Web backend/frontend MagiCore biết (source vẫn từ registry layer; danh
/// sách chỉ trả lời "framework này có được hỗ trợ không?").
/// Backend ids MagiCore can scaffold or materialize for the Web core.
///
/// Two families live here and both must be listed or a supported combination
/// becomes unreachable through `mgc create-web`:
/// - languages with a scaffold record (Express, NestJS, Spring, Quarkus,
///   Django, Laravel) — these run the framework's own initializer;
/// - languages the scaffold writes from a MagiCore-owned layer instead
///   (the remaining JS/Python/Go/Rust/.NET/PHP backends).
///
/// `django` and `laravel` were previously absent while their initializers and
/// dev launchers existed, so `is_supported("web", "django")` returned false even
/// though `mgc create-web django` worked. The fullstack ids were absent for the
/// same reason.
/// (Hai nhóm backend Web: có record initializer và có layer do MagiCore sở hữu.
/// Thiếu `django`/`laravel`/id fullstack trước đây làm `is_supported` trả false
/// trong khi lệnh create-web vẫn chạy được.)
pub const WEB_BACKEND: &[&str] = &[
    "actix-web",
    "axum",
    "django",
    "dotnet",
    "echo",
    "express",
    "fastapi",
    "fiber",
    "flask",
    "fastify",
    "gin",
    "hono",
    "koa",
    "laravel",
    "nest",
    "nestjs",
    "quarkus",
    "spring",
    "spring-boot",
    "symfony",
    "trpc",
];

/// Aliases `normalize_web_framework` maps onto a canonical id.
///
/// A gate that ran *after* normalisation would accept these, but it runs at the
/// CLI boundary on the user's literal word — so the registry must list the
/// aliases too, or `create-web ts` is rejected while the product supports it.
/// (Alias mà `normalize_web_framework` quy về id chuẩn. Cổng chạy tại CLI
/// boundary trên đúng chữ người dùng gõ, nên registry phải liệt kê cả alias,
/// nếu không `create-web ts` bị từ chối trong khi sản phẩm vẫn hỗ trợ.)
pub const WEB_ALIAS: &[&str] = &[
    "backend",
    "custom",
    "frontend",
    "go",
    "fullstack",
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
    "react-fastify",
    "react-fiber",
    "react-flask",
    "react-gin",
    "react-hono",
    "react-nestjs",
    "react-quarkus",
    "react-spring",
    "react-symfony",
    "react-trpc",
    "svelte-axum",
    "svelte-express",
    "svelte-fastapi",
    "svelte-gin",
    "svelte-hono",
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
    "vue-hono",
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
/// Ecosystem project local cho core id của CLI (`clo` được lưu là `cloud`).
pub fn ecosystem_for_core(core: &str) -> &str {
    if core == "clo" { "cloud" } else { core }
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

/// Whether `core`/`framework` is a known pair.
/// Cặp `core`/`framework` có được biết không.
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

/// Every framework id the registry accepts for `core`, in stable order.
///
/// Used by `create-*` to fail closed with an actionable list instead of
/// scaffolding metadata for a name nobody can build.
/// (Toàn bộ id registry chấp nhận cho `core`, thứ tự ổn định — để `create-*`
/// fail-closed kèm danh sách thay vì scaffold metadata cho tên không build được.)
pub fn supported_frameworks(core: &str) -> &'static [&'static str] {
    match core {
        "game" => GAME,
        "clo" | "cloud" => CLOUD,
        "cicd" => CICD,
        "iot" => IOT,
        "app" => APP,
        "lib" | "library" => LIB,
        "ai" => AI,
        "hardware" => HARDWARE,
        "web" => WEB_ALL,
        _ => &[],
    }
}

/// Web ids from all three families, concatenated for diagnostics.
/// Id Web của cả ba nhóm, nối lại để báo lỗi.
pub const WEB_ALL: &[&str] = &[
    // Frontend — xem WEB_FRONTEND. Giữ khớp tuyệt đối với bảng bên trên.
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
    // Backend — xem WEB_BACKEND.
    "actix-web",
    "axum",
    "django",
    "dotnet",
    "echo",
    "express",
    "fastapi",
    "fiber",
    "flask",
    "fastify",
    "gin",
    "hono",
    "koa",
    "laravel",
    "nest",
    "nestjs",
    "quarkus",
    "spring",
    "spring-boot",
    "symfony",
    "trpc",
    // Fullstack — xem WEB_FULLSTACK.
    "react-actix-web",
    "react-axum",
    "react-django",
    "react-echo",
    "react-express",
    "react-fastapi",
    "react-fastify",
    "react-fiber",
    "react-flask",
    "react-gin",
    "react-hono",
    "react-nestjs",
    "react-quarkus",
    "react-spring",
    "react-symfony",
    "react-trpc",
    "svelte-axum",
    "svelte-express",
    "svelte-fastapi",
    "svelte-gin",
    "svelte-hono",
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
    "vue-hono",
    "vue-laravel",
    "vue-nestjs",
    "vue-quarkus",
    "vue-symfony",
];

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
