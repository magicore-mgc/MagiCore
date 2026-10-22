use clap::Parser;

/// `mgc template` subcommands
#[derive(Debug, Parser, Clone)]
pub enum TemplateCmd {
    /// Fetch template layer từ registry về ~/.mgc/templates (create sẽ tự thấy)
    #[command(about = "Fetch template layer from registry into cache ~/.mgc/templates")]
    Fetch(TemplateFetchArgs),
    /// Publish kernel template thành package mgc-create-<core>-<name>
    #[command(about = "Publish kernel template layer to registry")]
    Publish(TemplatePublishArgs),
    /// Publish mọi layer web có template.toml+sources (registry-first source of truth)
    #[command(about = "Publish every template layer under templates/web to registry")]
    PublishAll,
}

/// `mgc template fetch <core> <name> [--registry URL] [--tag latest]`
#[derive(Debug, Parser, Clone)]
pub struct TemplateFetchArgs {
    pub core: String,
    pub name: String,
    #[arg(long, help = "override registry URL")]
    pub registry: Option<String>,
    #[arg(long, help = "dist-tag (default: latest)")]
    pub tag: Option<String>,
}

/// `mgc template publish <core> <name> [--registry URL] [--version]`
#[derive(Debug, Parser, Clone)]
pub struct TemplatePublishArgs {
    pub core: String,
    pub name: String,
    #[arg(long, help = "override registry URL")]
    pub registry: Option<String>,
    #[arg(long, help = "package version (default: 0.1.0)")]
    pub version: Option<String>,
}
