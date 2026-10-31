//! CommonCommand implementation owner — stores shared CLI command payloads and options.
//! Owner lưu payload và tùy chọn của các lệnh CLI dùng chung.

pub enum CommonCommand {
    Init {
        template: Option<String>,
        name: Option<String>,
        yes: bool,
        signature: Option<String>,
    },
    Dev {
        host: Option<String>,
        port: Option<u16>,
        clear: bool,
        compat_runtime: Option<String>,
        compat_runner: Option<String>,
    },
    Adopt {
        core: String,
        path: Option<std::path::PathBuf>,
    },
    Version {
        json: bool,
    },
    Info {
        package: String,
        json: bool,
    },
    Search {
        query: String,
        json: bool,
        exact: bool,
        page: Option<u32>,
    },
    Outdated {
        json: bool,
    },
    Audit {
        cmd: Option<crate::commands::definitions::AuditCmd>,
        fix: bool,
        format: Option<String>,
    },
    SelfUpdate {
        version: Option<String>,
        variant: Option<String>,
        dry_run: bool,
        trust_root: Vec<String>,
        allow_unsigned: bool,
    },
    SignRelease {
        manifest: Option<String>,
        key_hex: Option<String>,
    },
    Config {
        cmd: crate::commands::config::ConfigCmd,
        local: bool,
    },
    Stage {
        dir: Option<std::path::PathBuf>,
    },
    Import {
        dir: Option<std::path::PathBuf>,
        allow_unsigned: bool,
    },
    Migrate {
        cmd: crate::commands::migrate::MigrateCmd,
    },
    Sbom {
        format: Option<String>,
        output: Option<std::path::PathBuf>,
        name: Option<String>,
        version: Option<String>,
        dir: Option<std::path::PathBuf>,
    },
    Run {
        script: String,
        args: Vec<String>,
        compat_runtime: Option<String>,
        compat_runner: Option<String>,
    },
    Test {
        args: Vec<String>,
        compat_runtime: Option<String>,
        compat_runner: Option<String>,
    },
    Optimizer {
        force: bool,
    },
    Build {
        target: Option<String>,
        compat_runtime: Option<String>,
        compat_runner: Option<String>,
        engine: Option<String>,
        allow_output_warnings: bool,
        offline: bool,
    },
    Flash {
        board: Option<String>,
        skip_build: bool,
    },
    Deploy {
        run: bool,
    },
    CiGenerate,
    /// Verify an SBOM signature produced by `mgc build` (SEC-15).
    /// Verify chữ ký SBOM do `mgc build` tạo (SEC-15).
    VerifySbom {
        artifact: String,
        trust_key: String,
    },
    Verify,
    Start,
    Exec {
        command: String,
        args: Vec<String>,
        compat_runner: Option<String>,
    },
    Dlx {
        package: String,
        bin: Option<String>,
        allow_scripts: bool,
        args: Vec<String>,
    },
    Tool {
        cmd: crate::commands::definitions::ToolCmd,
    },
    Cache {
        action: String,
        target: String,
        yes: bool,
        dry_run: bool,
        ci: bool,
    },
    Link {
        package: Option<String>,
    },
    Unlink {
        package: Option<String>,
    },
    Why {
        package: String,
    },
    Publish {
        protocol: String,
        package: Option<String>,
        version: Option<String>,
        artifacts: Vec<std::path::PathBuf>,
        image: Option<String>,
        tag: Option<String>,
        access: Option<String>,
        dry_run: bool,
        json: bool,
        otp: Option<String>,
        force: bool,
        ignore_scripts: bool,
        allow_scripts: bool,
        no_git_checks: bool,
        publish_branch: Option<String>,
        batch: bool,
        report_summary: bool,
        patch: bool,
        minor: bool,
        major: bool,
        registry: Option<String>,
        token: Option<String>,
        trusted: bool,
        trusted_audience: Option<String>,
    },
    Patch {
        cmd: crate::commands::patch::PatchCmd,
    },
    Dedupe {
        dry_run: bool,
        prefer_latest: bool,
        json: bool,
    },
    Store {
        cmd: crate::commands::store::StoreCmd,
    },
    Bench {
        args: crate::commands::bench::BenchArgs,
    },
    Trust {
        cmd: crate::commands::trust::TrustCmd,
    },
    Hooks {
        cmd: crate::commands::hooks::HooksCmd,
    },
    Docs {
        output: Option<std::path::PathBuf>,
    },
    Completion {
        shell: crate::commands::completion::CompletionShell,
    },
    Telemetry {
        cmd: crate::commands::telemetry::TelemetryCmd,
    },
    Network {
        cmd: crate::commands::network::NetworkCmd,
    },
    Doctor {
        cmd: crate::commands::doctor::DoctorCmd,
    },
    Workspace {
        cmd: crate::commands::workspace::WorkspaceCmd,
    },
    Login {
        registry: Option<String>,
        username: Option<String>,
        password_stdin: bool,
        local: bool,
    },
    Registry {
        cmd: crate::commands::registry::RegistryCmd,
    },
    Model {
        cmd: crate::commands::model::ModelCmd,
    },
    Mcp,
    Capabilities,
}
