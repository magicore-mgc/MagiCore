//! Web manifest enrichment after scaffolding.
//! Bổ sung manifest Web sau khi scaffold.

#[cfg(feature = "web")]
use crate::commands::core::web::{enrich_web_project_manifest, parse_framework_request};
use crate::commands::web_scaffold_flags::WebScaffoldFlags;
use crate::wizard::engine::ScaffoldConfig;
use anyhow::Result;
use std::path::Path;

#[cfg(feature = "web")]
pub(in crate::commands::init) async fn seed_web_deps(
    project_dir: &Path,
    config: &ScaffoldConfig,
) -> Result<()> {
    let mut flags = WebScaffoldFlags::default();
    for feat in &config.features {
        match feat.as_str() {
            "typescript" | "ts" => flags.ts = true,
            "tailwindcss" | "tailwind" => flags.tailwindcss = true,
            "eslint" => flags.eslint = true,
            "vitest" => flags.vitest = true,
            "playwright" => flags.playwright = true,
            _ => {}
        }
    }
    if config.sub_type == "monorepo" {
        flags.monorepo = true;
    }
    let fe = &config.frameworks[0];
    let frontend = parse_framework_request(fe);
    match config.sub_type.as_str() {
        "fullstack" => {
            let backend = if config.frameworks.len() > 1 {
                Some(parse_framework_request(&config.frameworks[1]))
            } else {
                None
            };
            enrich_web_project_manifest(project_dir, &frontend, backend.as_ref(), &flags).await
        }
        "monorepo" => {
            let backend = config.frameworks.get(1).map(|b| parse_framework_request(b));
            enrich_web_project_manifest(project_dir, &frontend, backend.as_ref(), &flags).await
        }
        _ => enrich_web_project_manifest(project_dir, &frontend, None, &flags).await,
    }
}

#[cfg(not(feature = "web"))]
pub(in crate::commands::init) async fn seed_web_deps(
    _project_dir: &Path,
    _config: &ScaffoldConfig,
) -> Result<()> {
    // Nothing to seed without the web adapter; scaffolding of non-web cores
    // never reaches this function.
    // Không có gì để seed khi thiếu web adapter; scaffold core khác không gọi.
    Ok(())
}
