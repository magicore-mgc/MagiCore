//! Maps core/framework lanes to MagiCore-owned native store roots.
//! Ánh xạ lane core/framework sang store root native của MagiCore.

use anyhow::Result;
use std::path::PathBuf;

/// Store health route backed by the actual native installer for a lane.
/// Route kiểm tra store theo installer native thực sự của lane.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(in crate::commands::store) enum StoreRoute {
    WebProject,
    Global(&'static [StoreRootKind]),
}

/// MagiCore-managed roots written or consumed by native installers.
/// Các root do installer native của MagiCore ghi hoặc sử dụng.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(in crate::commands::store) enum StoreRootKind {
    ContentCas,
    Cargo,
    Go,
    Maven,
    Nuget,
    Pypi,
    Pub,
    Swift,
}

impl StoreRootKind {
    pub(in crate::commands::store) fn name(self) -> &'static str {
        match self {
            Self::ContentCas => "content-cas",
            Self::Cargo => "cargo",
            Self::Go => "go",
            Self::Maven => "maven",
            Self::Nuget => "nuget",
            Self::Pypi => "pypi",
            Self::Pub => "pub",
            Self::Swift => "swift",
        }
    }
}

pub(in crate::commands::store) const WEB_NATIVE_FRAMEWORKS: &[&str] = &[
    "angular",
    "astro",
    "express",
    "fastify",
    "hono",
    "nestjs",
    "nextjs",
    "node",
    "nuxt",
    "qwik",
    "react-vite",
    "remix",
    "solidjs",
    "sveltekit",
    "trpc",
    "ts",
    "vanilla",
    "vue-vite",
];
pub(in crate::commands::store) const AI_PYTHON_ROOTS: &[StoreRootKind] =
    &[StoreRootKind::ContentCas, StoreRootKind::Pypi];
pub(in crate::commands::store) const APP_FLUTTER_ROOTS: &[StoreRootKind] =
    &[StoreRootKind::ContentCas, StoreRootKind::Pub];
pub(in crate::commands::store) const APP_SWIFT_ROOTS: &[StoreRootKind] =
    &[StoreRootKind::ContentCas, StoreRootKind::Swift];
pub(in crate::commands::store) const LIB_CARGO_ROOTS: &[StoreRootKind] =
    &[StoreRootKind::ContentCas, StoreRootKind::Cargo];
pub(in crate::commands::store) const LIB_GO_ROOTS: &[StoreRootKind] =
    &[StoreRootKind::ContentCas, StoreRootKind::Go];
pub(in crate::commands::store) const LIB_MAVEN_ROOTS: &[StoreRootKind] =
    &[StoreRootKind::ContentCas, StoreRootKind::Maven];
pub(in crate::commands::store) const LIB_NUGET_ROOTS: &[StoreRootKind] =
    &[StoreRootKind::ContentCas, StoreRootKind::Nuget];
pub(in crate::commands::store) const LIB_PYPI_ROOTS: &[StoreRootKind] =
    &[StoreRootKind::ContentCas, StoreRootKind::Pypi];

/// Resolve only store routes currently implemented by native installers.
/// Chỉ phân giải route store hiện được installer native triển khai.
pub(in crate::commands::store) fn store_route(
    core: &str,
    framework: Option<&str>,
) -> Option<StoreRoute> {
    match (core, framework) {
        ("web", None) => Some(StoreRoute::WebProject),
        ("web", Some(name)) if WEB_NATIVE_FRAMEWORKS.contains(&name) => {
            Some(StoreRoute::WebProject)
        }
        ("web", Some("actix-web" | "axum" | "rust")) => Some(StoreRoute::Global(LIB_CARGO_ROOTS)),
        ("web", Some("django" | "fastapi" | "flask" | "python")) => {
            Some(StoreRoute::Global(LIB_PYPI_ROOTS))
        }
        ("web", Some("echo" | "fiber" | "gin" | "go")) => Some(StoreRoute::Global(LIB_GO_ROOTS)),
        ("web", Some("java" | "quarkus" | "spring-boot")) => {
            Some(StoreRoute::Global(LIB_MAVEN_ROOTS))
        }
        ("web", Some("dotnet" | "dotnet-minimal" | "dotnet-webapi")) => {
            Some(StoreRoute::Global(LIB_NUGET_ROOTS))
        }
        ("clo", Some("cdk" | "pulumi")) | ("lib", Some("ts")) => Some(StoreRoute::WebProject),
        ("ai", Some("python-agent")) => Some(StoreRoute::Global(AI_PYTHON_ROOTS)),
        ("app", Some("flutter")) => Some(StoreRoute::Global(APP_FLUTTER_ROOTS)),
        ("app", Some("swift")) => Some(StoreRoute::Global(APP_SWIFT_ROOTS)),
        ("lib", Some("rust")) | ("game", Some("bevy")) | ("iot", Some("esp32-rust")) => {
            Some(StoreRoute::Global(LIB_CARGO_ROOTS))
        }
        ("lib", Some("go")) => Some(StoreRoute::Global(LIB_GO_ROOTS)),
        ("lib", Some("java")) => Some(StoreRoute::Global(LIB_MAVEN_ROOTS)),
        ("lib", Some("dotnet")) => Some(StoreRoute::Global(LIB_NUGET_ROOTS)),
        ("lib", Some("python")) => Some(StoreRoute::Global(LIB_PYPI_ROOTS)),
        _ => None,
    }
}

/// Resolve an installer root without creating or modifying it.
/// Tìm root installer mà không tạo hay sửa dữ liệu.
pub(in crate::commands::store) fn store_root_path(kind: StoreRootKind) -> Result<PathBuf> {
    let global = mgc_platform::paths::GlobalPaths::new()?;
    let path = match kind {
        StoreRootKind::ContentCas => mgc_store::default_store_root(),
        StoreRootKind::Cargo => global.store.join("cargo"),
        StoreRootKind::Go => global.store.join("go"),
        StoreRootKind::Maven => global.store.join("maven"),
        StoreRootKind::Nuget => global.store.join("nuget"),
        StoreRootKind::Pypi => global.store.join("pypi"),
        StoreRootKind::Pub => mgc_platform::paths::mgc_data_dir()
            .join("store")
            .join("pub"),
        StoreRootKind::Swift => std::env::var("MGC_SWIFT_STORE_ROOT")
            .ok()
            .filter(|value| !value.trim().is_empty())
            .map(PathBuf::from)
            .unwrap_or_else(|| {
                mgc_platform::paths::mgc_data_dir()
                    .join("store")
                    .join("swift")
            }),
    };
    Ok(path)
}
