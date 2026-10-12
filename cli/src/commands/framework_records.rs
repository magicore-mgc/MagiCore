//! Framework qualification records — every wizard-selectable framework
//! id gets exactly one record stating what MagiCore may honestly claim
//! about it. Framework route and dependency-operation ownership are
//! separate: the latter is derived from `dep_gate::owner_for` for every
//! operation, never summarized as one framework-wide label.
//! A coverage test below parses `cli/src/wizard/*.rs` and fails when any
//! `Answer::new(.., "<id>")` lacks a record — new wizard entries cannot
//! silently arrive unqualified.
//!
//! (Hồ sơ qualification framework — mỗi id wizard chọn được có đúng một
//! record nêu điều MagiCore được phép claim trung thực. Test coverage
//! parse source wizard và fail khi id nào thiếu record.)

/// Route category for the framework catalog. Lifecycle evidence is reported
/// per record, and dependency ownership is reported separately per operation.
/// (Phân loại route của catalog; E2E và quyền dependency được báo riêng.)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FrameworkStatus {
    /// Offered by the wizard without an explicit `mgc-engine-path` route label.
    /// This label does not negate lifecycle evidence in the record.
    /// (Wizard có lựa chọn nhưng không mang nhãn route engine tường minh;
    /// bằng chứng lifecycle vẫn nằm trong record.)
    ScaffoldOnly,
    /// The framework branch routes through an MGC-owned engine path; this
    /// alone is not a claim that every dependency operation is supported.
    NativeEngine,
}

/// One qualification record: (core, wizard framework id) → claim.
/// (Một record qualification.)
pub struct FrameworkRecord {
    pub core: &'static str,
    pub framework: &'static str,
    pub status: FrameworkStatus,
    pub evidence: &'static str,
}

/// Shared evidence pointers (precise test/lane paths, no hand-waving).
/// (Căn cứ chung.)
pub const EV_WIZARD: &str = "wizard answer id only (lifecycle unqualified beyond scaffold)";
pub const EV_SCAFFOLD_WEB: &str =
    "cli/tests/web_fw_* + templates/backend.rs (scaffold proven, lifecycle unqualified)";
pub const EV_DEPGATE: &str =
    "dep_gate owner table + cli/tests/dep_gate_canary (zero-spawn proof per lane)";
pub const EV_LIB_PIPELINE: &str =
    "adapters/lib/src/install (spawn-free scan) + mgc-resolver protocols_* tests + dep_gate table";
pub const EV_WEB_ENGINE: &str = "MGC-owned JS web dependency pipeline (resolve/lock/fetch/CAS/materialize) + dep_gate web Native cell; native lifecycle E2E for all 18 JS web frameworks on macOS arm64 in debug and release profiles (`cli/tests/web_fw_native_matrix.rs`, 11 lanes; `cli/tests/web_fw_fe_*.rs`, 7 lanes), covering install/frozen replay/test/build when defined/dev HTTP readiness/process and port cleanup. Dependency ownership remains reported per operation.";
pub const EV_RN_GATE: &str =
    "dep_gate app/rn Unsupported cell + dep_gate_canary rn_* (no runner exists)";

/// Evidence from prior E2E runs. Historical spawn evidence does not prove
/// current dependency ownership; operation cells below come from dep_gate.
/// (E2E lịch sử không chứng minh ownership hiện tại; operation lấy từ dep_gate.)
pub const EV_E2E_APP_FLUTTER: &str = "Current native dependency E2E: `add-app fixture_pkg` resolves, verifies, writes pubspec+mgc.lock and installs the graph with zero Flutter/Dart spawn; Flutter list/dev and dependency-health outdated scan remain toolchain-owned. Historical delegated `flutter pub add` evidence is not current ownership proof.";
pub const EV_E2E_APP_SWIFT: &str = "Historical SwiftPM spawn is not current ownership evidence; see per-operation owner cells derived from dep_gate";
pub const EV_E2E_CLO_TERRAFORM: &str = "Terraform dependency operations are currently unsupported; deploy/tool execution is a separate capability and does not imply native package ownership";
pub const EV_E2E_HARDWARE: &str = "E2E template lane: create/add/install/list all exit 0 (optimizer.json/bench.json materialized); `mgc optimizer` applied profile.json + runtime envs on web+ai projects; `mgc bench` timed install; no package lifecycle by design";
pub const EV_E2E_IOT_PIO: &str = "Historical PlatformIO spawn is not current ownership evidence; current dependency operations are unsupported";
pub const EV_E2E_IOT_WEST: &str = "Historical West spawn is not current ownership evidence; current dependency operations are unsupported";
pub const EV_E2E_WEB_JS: &str = "Native JS web lifecycle E2E: 18 frameworks on macOS arm64 in debug and release profiles; `cli/tests/web_fw_native_matrix.rs` (11 lanes) + `cli/tests/web_fw_fe_*.rs` (7 lanes), covering install/frozen replay/test/build when defined/dev HTTP readiness/process and port cleanup. Dependency ownership remains reported per operation.";
pub const EV_E2E_WEB_BACKEND: &str = "Empty-graph install/frozen route E2E on V3 and V4 for all 12 advertised non-JS web backend frameworks (Actix/Axum, Django/FastAPI/Flask, Echo/Fiber/Gin, Quarkus/Spring Boot, .NET Minimal/Web API) plus five generic language aliases (Rust, Python, Go, Java, .NET); non-empty add/install/frozen replay against hermetic local registries is exercised for Python, Rust, Go, Java, and .NET on V3 and V4. Both paths preserve the .mgc.core web marker. Shared-store root routing is covered by a separate read-only store-doctor E2E. Web security-policy selection is covered separately by an adapter unit test. This does not qualify framework dev/build/test runtime.";

/// Every wizard-selectable framework id, exactly once per core.
/// (Mọi id framework wizard chọn được, đúng một lần mỗi core.)
pub const RECORDS: &[FrameworkRecord] = &[
    // ── ai ──
    FrameworkRecord {
        core: "ai",
        framework: "python-agent",
        status: FrameworkStatus::ScaffoldOnly,
        evidence: EV_DEPGATE,
    },
    FrameworkRecord {
        core: "ai",
        framework: "mcp-server",
        status: FrameworkStatus::ScaffoldOnly,
        evidence: EV_WIZARD,
    },
    // ── app ──
    FrameworkRecord {
        core: "app",
        framework: "flutter",
        status: FrameworkStatus::ScaffoldOnly,
        evidence: EV_E2E_APP_FLUTTER,
    },
    FrameworkRecord {
        core: "app",
        framework: "kotlin",
        status: FrameworkStatus::ScaffoldOnly,
        evidence: "dep_gate app/kotlin install+list cells + canary (add/remove/update: no runner)",
    },
    FrameworkRecord {
        core: "app",
        framework: "swift",
        status: FrameworkStatus::ScaffoldOnly,
        evidence: EV_E2E_APP_SWIFT,
    },
    FrameworkRecord {
        core: "app",
        framework: "react-native",
        status: FrameworkStatus::ScaffoldOnly,
        evidence: EV_RN_GATE,
    },
    FrameworkRecord {
        core: "app",
        framework: "maui",
        status: FrameworkStatus::ScaffoldOnly,
        evidence: EV_WIZARD,
    },
    FrameworkRecord {
        core: "app",
        framework: "tauri",
        status: FrameworkStatus::ScaffoldOnly,
        evidence: EV_WIZARD,
    },
    FrameworkRecord {
        core: "app",
        framework: "multi",
        status: FrameworkStatus::ScaffoldOnly,
        evidence: "wizard branch node (per-platform lanes gate individually)",
    },
    // ── cicd: scaffold/pipeline only by design ──
    FrameworkRecord {
        core: "cicd",
        framework: "argocd",
        status: FrameworkStatus::ScaffoldOnly,
        evidence: EV_WIZARD,
    },
    FrameworkRecord {
        core: "cicd",
        framework: "aws",
        status: FrameworkStatus::ScaffoldOnly,
        evidence: EV_WIZARD,
    },
    FrameworkRecord {
        core: "cicd",
        framework: "cloudflare",
        status: FrameworkStatus::ScaffoldOnly,
        evidence: EV_WIZARD,
    },
    FrameworkRecord {
        core: "cicd",
        framework: "gcp",
        status: FrameworkStatus::ScaffoldOnly,
        evidence: EV_WIZARD,
    },
    FrameworkRecord {
        core: "cicd",
        framework: "github-actions",
        status: FrameworkStatus::ScaffoldOnly,
        evidence: EV_WIZARD,
    },
    // ── cloud ──
    FrameworkRecord {
        core: "clo",
        framework: "terraform",
        status: FrameworkStatus::ScaffoldOnly,
        evidence: EV_E2E_CLO_TERRAFORM,
    },
    FrameworkRecord {
        core: "clo",
        framework: "cdk",
        status: FrameworkStatus::NativeEngine,
        evidence: "clo lane branches pre-gate into the embedded web engine",
    },
    FrameworkRecord {
        core: "clo",
        framework: "pulumi",
        status: FrameworkStatus::NativeEngine,
        evidence: "clo lane branches pre-gate into the embedded web engine",
    },
    FrameworkRecord {
        core: "clo",
        framework: "cloudflare",
        status: FrameworkStatus::ScaffoldOnly,
        evidence: EV_WIZARD,
    },
    // ── game: only bevy has a runner (cargo); the rest fail closed ──
    FrameworkRecord {
        core: "game",
        framework: "bevy",
        status: FrameworkStatus::ScaffoldOnly,
        evidence: EV_DEPGATE,
    },
    FrameworkRecord {
        core: "game",
        framework: "godot",
        status: FrameworkStatus::ScaffoldOnly,
        evidence: "no package manager (lane fail-closed); canary godot_*",
    },
    FrameworkRecord {
        core: "game",
        framework: "unity",
        status: FrameworkStatus::ScaffoldOnly,
        evidence: "UPM CLI is P2 (lane fail-closed)",
    },
    FrameworkRecord {
        core: "game",
        framework: "unreal",
        status: FrameworkStatus::ScaffoldOnly,
        evidence: "no package manager (lane fail-closed)",
    },
    // ── hardware: inventory/templates only ──
    FrameworkRecord {
        core: "hardware",
        framework: "bench",
        status: FrameworkStatus::ScaffoldOnly,
        evidence: EV_E2E_HARDWARE,
    },
    FrameworkRecord {
        core: "hardware",
        framework: "optimizer",
        status: FrameworkStatus::ScaffoldOnly,
        evidence: EV_E2E_HARDWARE,
    },
    // ── iot ──
    FrameworkRecord {
        core: "iot",
        framework: "esp32-rust",
        status: FrameworkStatus::ScaffoldOnly,
        evidence: EV_DEPGATE,
    },
    FrameworkRecord {
        core: "iot",
        framework: "platformio",
        status: FrameworkStatus::ScaffoldOnly,
        evidence: EV_E2E_IOT_PIO,
    },
    FrameworkRecord {
        core: "iot",
        framework: "zephyr-arm",
        status: FrameworkStatus::ScaffoldOnly,
        evidence: EV_E2E_IOT_WEST,
    },
    FrameworkRecord {
        core: "iot",
        framework: "esp32",
        status: FrameworkStatus::ScaffoldOnly,
        evidence: "board variant (framework resolved at detect time from markers)",
    },
    FrameworkRecord {
        core: "iot",
        framework: "esp32c3",
        status: FrameworkStatus::ScaffoldOnly,
        evidence: "board variant (framework resolved at detect time from markers)",
    },
    FrameworkRecord {
        core: "iot",
        framework: "esp32dev",
        status: FrameworkStatus::ScaffoldOnly,
        evidence: "board variant (framework resolved at detect time from markers)",
    },
    FrameworkRecord {
        core: "iot",
        framework: "esp32s3",
        status: FrameworkStatus::ScaffoldOnly,
        evidence: "board variant (framework resolved at detect time from markers)",
    },
    FrameworkRecord {
        core: "iot",
        framework: "nodemcu-32s",
        status: FrameworkStatus::ScaffoldOnly,
        evidence: "board variant (framework resolved at detect time from markers)",
    },
    FrameworkRecord {
        core: "iot",
        framework: "nrf52dk_nrf52832",
        status: FrameworkStatus::ScaffoldOnly,
        evidence: "board variant (framework resolved at detect time from markers)",
    },
    FrameworkRecord {
        core: "iot",
        framework: "stm32f4_disc",
        status: FrameworkStatus::ScaffoldOnly,
        evidence: "board variant (framework resolved at detect time from markers)",
    },
    // ── lib: operation ownership is emitted from the firewall per language ──
    FrameworkRecord {
        core: "lib",
        framework: "ts",
        status: FrameworkStatus::NativeEngine,
        evidence: EV_LIB_PIPELINE,
    },
    FrameworkRecord {
        core: "lib",
        framework: "rust",
        status: FrameworkStatus::NativeEngine,
        evidence: "Rust resolver/CAS install path has local E2E; Cargo.lock synchronization for MGC-native add/update is not yet proven, so this record does not claim production-ready parity",
    },
    FrameworkRecord {
        core: "lib",
        framework: "python",
        status: FrameworkStatus::NativeEngine,
        evidence: "Python native path is manifest/artifact constrained; unsupported source/wheel/marker cases must fail closed; per-operation claims are emitted from dep_gate",
    },
    FrameworkRecord {
        core: "lib",
        framework: "go",
        status: FrameworkStatus::NativeEngine,
        evidence: "MGC-native Go dependency lane requires go.mod; no Go scaffold is currently offered by the Lib wizard",
    },
    FrameworkRecord {
        core: "lib",
        framework: "java",
        status: FrameworkStatus::NativeEngine,
        evidence: "MGC-native Maven lane requires pom.xml; Gradle and Java scaffold generation are not currently offered by the Lib wizard",
    },
    FrameworkRecord {
        core: "lib",
        framework: "dotnet",
        status: FrameworkStatus::NativeEngine,
        evidence: "MGC-native NuGet lane requires exactly one root csproj; .NET scaffold generation is not currently offered by the Lib wizard",
    },
    // ── web: vanilla/TS/Node have explicit engine route labels; remaining
    // wizard framework IDs use the scaffold route label. Lifecycle evidence
    // is recorded separately and does not change per-operation ownership. ──
    FrameworkRecord {
        core: "web",
        framework: "vanilla",
        status: FrameworkStatus::NativeEngine,
        evidence: EV_WEB_ENGINE,
    },
    FrameworkRecord {
        core: "web",
        framework: "ts",
        status: FrameworkStatus::NativeEngine,
        evidence: EV_WEB_ENGINE,
    },
    FrameworkRecord {
        core: "web",
        framework: "node",
        status: FrameworkStatus::NativeEngine,
        evidence: EV_WEB_ENGINE,
    },
    FrameworkRecord {
        core: "web",
        framework: "actix-web",
        status: FrameworkStatus::ScaffoldOnly,
        evidence: EV_E2E_WEB_BACKEND,
    },
    FrameworkRecord {
        core: "web",
        framework: "angular",
        status: FrameworkStatus::ScaffoldOnly,
        evidence: EV_E2E_WEB_JS,
    },
    FrameworkRecord {
        core: "web",
        framework: "astro",
        status: FrameworkStatus::ScaffoldOnly,
        evidence: EV_E2E_WEB_JS,
    },
    FrameworkRecord {
        core: "web",
        framework: "axum",
        status: FrameworkStatus::ScaffoldOnly,
        evidence: EV_E2E_WEB_BACKEND,
    },
    FrameworkRecord {
        core: "web",
        framework: "backend",
        status: FrameworkStatus::ScaffoldOnly,
        evidence: "wizard branch node (lifecycle unqualified)",
    },
    FrameworkRecord {
        core: "web",
        framework: "custom",
        status: FrameworkStatus::ScaffoldOnly,
        evidence: EV_WIZARD,
    },
    FrameworkRecord {
        core: "web",
        framework: "django",
        status: FrameworkStatus::ScaffoldOnly,
        evidence: EV_E2E_WEB_BACKEND,
    },
    FrameworkRecord {
        core: "web",
        framework: "dotnet",
        status: FrameworkStatus::ScaffoldOnly,
        evidence: EV_E2E_WEB_BACKEND,
    },
    FrameworkRecord {
        core: "web",
        framework: "dotnet-minimal",
        status: FrameworkStatus::ScaffoldOnly,
        evidence: EV_E2E_WEB_BACKEND,
    },
    FrameworkRecord {
        core: "web",
        framework: "dotnet-webapi",
        status: FrameworkStatus::ScaffoldOnly,
        evidence: EV_E2E_WEB_BACKEND,
    },
    FrameworkRecord {
        core: "web",
        framework: "echo",
        status: FrameworkStatus::ScaffoldOnly,
        evidence: EV_E2E_WEB_BACKEND,
    },
    FrameworkRecord {
        core: "web",
        framework: "express",
        status: FrameworkStatus::ScaffoldOnly,
        evidence: EV_E2E_WEB_JS,
    },
    FrameworkRecord {
        core: "web",
        framework: "fastapi",
        status: FrameworkStatus::ScaffoldOnly,
        evidence: EV_E2E_WEB_BACKEND,
    },
    FrameworkRecord {
        core: "web",
        framework: "fastify",
        status: FrameworkStatus::ScaffoldOnly,
        evidence: EV_E2E_WEB_JS,
    },
    FrameworkRecord {
        core: "web",
        framework: "fiber",
        status: FrameworkStatus::ScaffoldOnly,
        evidence: EV_E2E_WEB_BACKEND,
    },
    FrameworkRecord {
        core: "web",
        framework: "flask",
        status: FrameworkStatus::ScaffoldOnly,
        evidence: EV_E2E_WEB_BACKEND,
    },
    FrameworkRecord {
        core: "web",
        framework: "frontend",
        status: FrameworkStatus::ScaffoldOnly,
        evidence: "wizard branch node (lifecycle unqualified)",
    },
    FrameworkRecord {
        core: "web",
        framework: "fullstack",
        status: FrameworkStatus::ScaffoldOnly,
        evidence: "wizard branch node (lifecycle unqualified)",
    },
    FrameworkRecord {
        core: "web",
        framework: "gin",
        status: FrameworkStatus::ScaffoldOnly,
        evidence: EV_E2E_WEB_BACKEND,
    },
    FrameworkRecord {
        core: "web",
        framework: "go",
        status: FrameworkStatus::ScaffoldOnly,
        evidence: EV_E2E_WEB_BACKEND,
    },
    FrameworkRecord {
        core: "web",
        framework: "hono",
        status: FrameworkStatus::ScaffoldOnly,
        evidence: EV_E2E_WEB_JS,
    },
    FrameworkRecord {
        core: "web",
        framework: "java",
        status: FrameworkStatus::ScaffoldOnly,
        evidence: EV_E2E_WEB_BACKEND,
    },
    FrameworkRecord {
        core: "web",
        framework: "laravel",
        status: FrameworkStatus::ScaffoldOnly,
        evidence: EV_SCAFFOLD_WEB,
    },
    FrameworkRecord {
        core: "web",
        framework: "monorepo",
        status: FrameworkStatus::ScaffoldOnly,
        evidence: "wizard branch node (lifecycle unqualified)",
    },
    FrameworkRecord {
        core: "web",
        framework: "nestjs",
        status: FrameworkStatus::ScaffoldOnly,
        evidence: EV_E2E_WEB_JS,
    },
    FrameworkRecord {
        core: "web",
        framework: "nextjs",
        status: FrameworkStatus::ScaffoldOnly,
        evidence: EV_E2E_WEB_JS,
    },
    FrameworkRecord {
        core: "web",
        framework: "nuxt",
        status: FrameworkStatus::ScaffoldOnly,
        evidence: EV_E2E_WEB_JS,
    },
    FrameworkRecord {
        core: "web",
        framework: "php",
        status: FrameworkStatus::ScaffoldOnly,
        evidence: "wizard language branch (lifecycle unqualified)",
    },
    FrameworkRecord {
        core: "web",
        framework: "python",
        status: FrameworkStatus::ScaffoldOnly,
        evidence: EV_E2E_WEB_BACKEND,
    },
    FrameworkRecord {
        core: "web",
        framework: "quarkus",
        status: FrameworkStatus::ScaffoldOnly,
        evidence: EV_E2E_WEB_BACKEND,
    },
    FrameworkRecord {
        core: "web",
        framework: "qwik",
        status: FrameworkStatus::ScaffoldOnly,
        evidence: EV_E2E_WEB_JS,
    },
    FrameworkRecord {
        core: "web",
        framework: "react-fastify",
        status: FrameworkStatus::ScaffoldOnly,
        evidence: EV_WIZARD,
    },
    FrameworkRecord {
        core: "web",
        framework: "react-spring",
        status: FrameworkStatus::ScaffoldOnly,
        evidence: "React + Spring Boot fullstack scaffold; dependency install is unsupported until both workspace lanes are MGC-owned",
    },
    FrameworkRecord {
        core: "web",
        framework: "react-vite",
        status: FrameworkStatus::ScaffoldOnly,
        evidence: EV_E2E_WEB_JS,
    },
    FrameworkRecord {
        core: "web",
        framework: "remix",
        status: FrameworkStatus::ScaffoldOnly,
        evidence: EV_E2E_WEB_JS,
    },
    FrameworkRecord {
        core: "web",
        framework: "rust",
        status: FrameworkStatus::ScaffoldOnly,
        evidence: EV_E2E_WEB_BACKEND,
    },
    FrameworkRecord {
        core: "web",
        framework: "solidjs",
        status: FrameworkStatus::ScaffoldOnly,
        evidence: EV_E2E_WEB_JS,
    },
    FrameworkRecord {
        core: "web",
        framework: "spring-boot",
        status: FrameworkStatus::ScaffoldOnly,
        evidence: EV_E2E_WEB_BACKEND,
    },
    FrameworkRecord {
        core: "web",
        framework: "sveltekit",
        status: FrameworkStatus::ScaffoldOnly,
        evidence: EV_E2E_WEB_JS,
    },
    FrameworkRecord {
        core: "web",
        framework: "symfony",
        status: FrameworkStatus::ScaffoldOnly,
        evidence: EV_SCAFFOLD_WEB,
    },
    FrameworkRecord {
        core: "web",
        framework: "trpc",
        status: FrameworkStatus::ScaffoldOnly,
        evidence: EV_E2E_WEB_JS,
    },
    FrameworkRecord {
        core: "web",
        framework: "vue-laravel",
        status: FrameworkStatus::ScaffoldOnly,
        evidence: EV_WIZARD,
    },
    FrameworkRecord {
        core: "web",
        framework: "vue-vite",
        status: FrameworkStatus::ScaffoldOnly,
        evidence: EV_E2E_WEB_JS,
    },
];

/// Machine string for the framework route, not dependency ownership.
/// (Chuỗi máy-đọc cho framework route, không phải quyền dependency.)
pub fn status_string(status: &FrameworkStatus) -> String {
    match status {
        FrameworkStatus::ScaffoldOnly => "scaffold-only".to_string(),
        FrameworkStatus::NativeEngine => "mgc-engine-path".to_string(),
    }
}

fn dependency_ecosystem(record: &FrameworkRecord) -> Option<&'static str> {
    use crate::commands::dep_gate::eco;
    match (record.core, record.framework) {
        ("ai", "python-agent") | ("lib", "python") => Some(eco::PYTHON),
        ("app", "flutter") => Some(eco::FLUTTER),
        ("app", "kotlin") => Some(eco::KOTLIN),
        ("app", "swift") => Some(eco::SWIFT),
        ("app", "react-native") => Some(eco::RN),
        ("app", "objc") => Some(eco::OBJC),
        ("lib", "ts") => Some(eco::TS),
        ("lib", "rust") => Some(eco::RUST),
        ("lib", "go") => Some(eco::GO),
        ("lib", "java") => Some(eco::JAVA),
        ("lib", "dotnet") => Some(eco::DOTNET),
        ("game", "bevy") => Some(eco::BEVY),
        ("iot", "esp32-rust") => Some("esp32-rust"),
        ("iot", "platformio") => Some("platformio"),
        ("iot", "zephyr-arm") => Some("zephyr"),
        ("clo", "terraform") => Some(eco::TERRAFORM),
        ("clo", "cdk" | "pulumi") => Some(eco::JS),
        // A framework-level capability row describes the whole generated
        // stack; React + Spring is not JavaScript-only dependency ownership.
        // (Row capability bao trọn stack được sinh, không chỉ frontend React.)
        ("web", "react-spring") => None,
        (
            "web",
            "angular" | "astro" | "express" | "fastify" | "hono" | "nestjs" | "nextjs" | "nuxt"
            | "qwik" | "react-vite" | "remix" | "solidjs" | "sveltekit" | "trpc" | "vanilla"
            | "vue-vite" | "node" | "ts",
        ) => Some(eco::JS),
        ("web", "actix-web" | "axum" | "rust") => Some(eco::RUST),
        ("web", "django" | "fastapi" | "flask" | "python") => Some(eco::PYTHON),
        ("web", "echo" | "fiber" | "gin" | "go") => Some(eco::GO),
        ("web", "java" | "quarkus" | "spring-boot") => Some(eco::JAVA),
        ("web", "dotnet" | "dotnet-minimal" | "dotnet-webapi") => Some(eco::DOTNET),
        _ => None,
    }
}

/// Manifest identity required by the owner table for language lanes whose
/// ecosystem alone is insufficient to prove MGC owns the dependency format.
/// (Manifest cụ thể bắt buộc cho lane mà tên ecosystem chưa đủ để chứng minh ownership.)
fn dependency_manifest_format(record: &FrameworkRecord) -> Option<&'static str> {
    match (record.core, record.framework) {
        ("ai", "python-agent") => Some("mgc-pyproject"),
        ("lib", "ts") => Some("package-json"),
        ("lib", "rust") => Some("cargo-toml"),
        ("lib", "python") => Some("pep621-native"),
        ("lib", "go") => Some("go-mod"),
        ("lib", "java") => Some("maven-pom"),
        ("lib", "dotnet") => Some("csproj"),
        ("clo", "cdk" | "pulumi") => Some("package-json"),
        ("game", "bevy") | ("iot", "esp32-rust") => Some("cargo-toml"),
        ("web", "actix-web" | "axum" | "rust") => Some("cargo-toml"),
        ("web", "django" | "fastapi" | "flask" | "python") => Some("pep621-native"),
        ("web", "echo" | "fiber" | "gin" | "go") => Some("go-mod"),
        ("web", "java" | "quarkus" | "spring-boot") => Some("maven-pom"),
        ("web", "dotnet" | "dotnet-minimal" | "dotnet-webapi") => Some("csproj"),
        _ => None,
    }
}

/// The web backend uses the native Lib language engine without transferring
/// project identity away from `web`.
/// Backend web dùng engine ngôn ngữ native của Lib nhưng không đổi owner project.
fn dependency_engine_core(record: &FrameworkRecord) -> &'static str {
    if record.core == "web"
        && dependency_ecosystem(record)
            .is_some_and(|ecosystem| !matches!(ecosystem, crate::commands::dep_gate::eco::JS))
    {
        "lib"
    } else {
        record.core
    }
}

fn dependency_ownership_json(record: &FrameworkRecord) -> serde_json::Value {
    use crate::commands::dep_gate::{DepContext, DepOp, DepOwner, owner_for};
    let ecosystem = dependency_ecosystem(record);
    let manifest_format = dependency_manifest_format(record);
    let engine_core = dependency_engine_core(record);
    let mut operations = serde_json::Map::new();
    for op in DepOp::ALL {
        let context = DepContext::new(engine_core, ecosystem, Some(record.framework), None, *op)
            .with_manifest_format(manifest_format);
        let owner = match owner_for(&context) {
            DepOwner::Native => "mgc-native",
            DepOwner::ScaffoldOnly => "scaffold-only",
            DepOwner::Unsupported => "unsupported",
        };
        let mut cell = serde_json::json!({"owner": owner});
        if matches!((record.core, record.framework), ("clo", "cdk" | "pulumi")) {
            cell["requires"] = serde_json::json!("package.json; embedded MGC JavaScript engine");
        }
        if matches!(
            (record.core, record.framework),
            ("game", "bevy") | ("iot", "esp32-rust")
        ) && owner == "mgc-native"
        {
            cell["requires"] = serde_json::json!("Cargo.toml");
        }
        if record.core == "web" && engine_core == "lib" && owner == "mgc-native" {
            cell["requires"] = serde_json::json!(
                "web project identity; native Lib language adapter; required_manifest"
            );
            cell["engine_core"] = serde_json::json!("lib");
        }
        operations.insert(op.as_str().to_string(), cell);
    }
    serde_json::Value::Object(operations)
}

/// All records for one core, as machine-readable JSON values (consumed by
/// `mgc capabilities` — the registry is live data, not a dead table).
/// (Mọi record một core dạng JSON.)
pub fn qualification_json(core: &str) -> serde_json::Value {
    let rows: Vec<serde_json::Value> = RECORDS
        .iter()
        .filter(|record| record.core == core)
        .map(|record| {
            let mut row = serde_json::json!({
                "framework": record.framework,
                "status": status_string(&record.status),
                "dependency_ownership": dependency_ownership_json(record),
                "evidence": record.evidence,
            });
            if let Some(format) = dependency_manifest_format(record) {
                row["required_manifest"] = serde_json::json!(format);
            }
            row
        })
        .collect();
    serde_json::Value::Array(rows)
}

#[cfg(test)]
#[path = "test/framework_records_test.rs"]
mod tests;
