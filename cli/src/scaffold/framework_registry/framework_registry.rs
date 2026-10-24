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
pub const AI: &[&str] = &[
    "pytorch",
    "tensorflow",
    "jax",
    "onnx",
    "candle",
    "mcp-server",
];
pub const HARDWARE: &[&str] = &["optimizer", "bench"];
/// Web backend/frontend ids known to MagiCore (source still comes from the
/// registry layer; this list only answers "is this framework supported?").
/// Id Web backend/frontend MagiCore biết (source vẫn từ registry layer; danh
/// sách chỉ trả lời "framework này có được hỗ trợ không?").
pub const WEB_BACKEND: &[&str] = &[
    "actix-web",
    "axum",
    "dotnet",
    "echo",
    "express",
    "fastapi",
    "fiber",
    "flask",
    "laravel",
    "nest",
    "nestjs",
    "quarkus",
    "spring",
    "symfony",
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
        ("web", "laravel") => Some(&["artisan"]),
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
        "web" => return WEB_BACKEND.contains(&framework) || WEB_FRONTEND.contains(&framework),
        _ => return false,
    };
    table.contains(&framework)
}
