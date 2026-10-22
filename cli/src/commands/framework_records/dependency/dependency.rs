//! Dependency ownership projection over framework records and dep gate.
//! Projection dependency ownership từ record framework và dep gate.

use super::super::FrameworkRecord;

pub(in crate::commands::framework_records::implementation) fn dependency_ecosystem(
    record: &FrameworkRecord,
) -> Option<&'static str> {
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
pub(in crate::commands::framework_records::implementation) fn dependency_manifest_format(
    record: &FrameworkRecord,
) -> Option<&'static str> {
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
pub(in crate::commands::framework_records::implementation) fn dependency_engine_core(
    record: &FrameworkRecord,
) -> &'static str {
    if record.core == "web"
        && dependency_ecosystem(record)
            .is_some_and(|ecosystem| !matches!(ecosystem, crate::commands::dep_gate::eco::JS))
    {
        "lib"
    } else {
        record.core
    }
}

pub(in crate::commands::framework_records::implementation) fn dependency_ownership_json(
    record: &FrameworkRecord,
) -> serde_json::Value {
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
