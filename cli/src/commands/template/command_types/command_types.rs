//! Argument types for the registry-layer fetch path.
//! Kiểu tham số cho đường tải registry layer.
//!
//! MagiCore publishes no template. This module survives only because
//! `ensure_layer` still resolves a registry layer at scaffold time, and the
//! resolution path needs the request shape. There is no `mgc template`
//! subcommand: a framework's own initializer is the only scaffolding source.
//! MagiCore không phát hành template. Module này còn lại chỉ vì `ensure_layer`
//! vẫn resolve registry layer lúc scaffold và đường resolve cần kiểu yêu cầu.
//! Không có subcommand `mgc template`: initializer của chính framework là nguồn
//! scaffold duy nhất.

use clap::Parser;

/// A request to resolve one layer from the configured registry.
/// Yêu cầu resolve một layer từ registry đã cấu hình.
#[derive(Debug, Parser, Clone)]
#[allow(dead_code)] // Fields carry the request shape the resolver reads; not all are consumed yet.
pub struct TemplateFetchArgs {
    pub core: String,
    pub name: String,
    #[arg(long, help = "override registry URL")]
    pub registry: Option<String>,
    #[arg(long, help = "dist-tag (default: latest)")]
    pub tag: Option<String>,
}
