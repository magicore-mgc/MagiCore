use crate::commands::definitions::AuditCmd;
// CommonCommands schema — khai báo nhóm lệnh CLI theo trách nhiệm.
use clap::Subcommand;

#[derive(Subcommand, Clone)]
#[allow(clippy::large_enum_variant)]
pub enum CommonCommands {
    // ── Common / Global commands ────────────────────────────────────────
    #[command(about = "Interactive project wizard")]
    Init {
        #[arg(short, long)]
        template: Option<String>,
        #[arg(
            long,
            help = "Write a plain-text core identity marker (.mgc.core); this is not a cryptographic signature"
        )]
        signature: Option<String>,
    },
    #[command(about = "Take ownership of a project another tool created")]
    Adopt {
        #[arg(long, help = "Core that owns the project (cicd, clo, game, iot)")]
        core: String,
        #[arg(
            long,
            value_name = "PATH",
            help = "Project directory (defaults to the current directory)"
        )]
        path: Option<std::path::PathBuf>,
    },
    #[command(about = "Print the MagiCore CLI version")]
    Version {
        #[arg(long, help = "Output as JSON")]
        json: bool,
    },
    #[command(about = "Show package information")]
    Info {
        package: String,
        #[arg(long, help = "Output as JSON")]
        json: bool,
    },
    #[command(about = "Search for packages")]
    Search {
        query: String,
        #[arg(long, help = "Output as JSON")]
        json: bool,
        #[arg(long, help = "Exact match search")]
        exact: bool,
        #[arg(long, help = "Page number (20 results per page)")]
        page: Option<u32>,
    },
    #[command(about = "Check outdated packages using the core's native resolver")]
    Outdated {
        #[arg(long, help = "Output as JSON")]
        json: bool,
    },
    #[command(about = "Audit packages for vulnerabilities")]
    Audit {
        #[command(subcommand)]
        cmd: Option<AuditCmd>,
        #[arg(
            long,
            help = "Bump vulnerable packages and rewrite lockfile on success"
        )]
        fix: bool,
        /// Machine-readable output format: table (default), json
        /// (versioned envelope), sarif (2.1.0 for GitHub Security),
        /// cyclonedx (1.5 Vulnerable BOM). Table renders the same data
        /// for humans; the others print ONLY the machine payload.
        /// Định dạng cho máy: table (mặc định), json (envelope có
        /// version), sarif (2.1.0 cho GitHub Security), cyclonedx (1.5
        /// Vulnerable BOM). Table hiển thị cho người; còn lại chỉ in
        /// payload máy.
        #[arg(long, help = "Output format: table|json|sarif|cyclonedx")]
        format: Option<String>,
    },
    #[command(about = "Update MagiCore CLI to the latest version")]
    SelfUpdate {
        #[arg(
            long,
            help = "target version (default: latest release), with or without leading 'v'"
        )]
        version: Option<String>,
        #[arg(
            long,
            help = "release variant: magicore|magicore-web|all (default: detected from this binary)"
        )]
        variant: Option<String>,
        #[arg(
            long,
            help = "print what would be downloaded without changing anything"
        )]
        dry_run: bool,
        #[arg(
            long = "trust-root",
            help = "hex Ed25519 pubkey pinning the release manifest (repeatable); or MGC_RELEASE_TRUST_ROOTS"
        )]
        trust_root: Vec<String>,
        #[arg(
            long = "allow-unsigned",
            help = "explicit opt-in to a sha256-only update with NO provenance (refused when any trust root is configured; loud warning)"
        )]
        allow_unsigned: bool,
    },
    #[command(
        about = "Sign a release manifest with the Ed25519 release key (ring backend, no network)"
    )]
    SignRelease {
        #[arg(
            long,
            help = "manifest file to sign (default: manifest.json); writes <manifest>.sig"
        )]
        manifest: Option<String>,
        #[arg(
            long = "key-hex",
            help = "64-hex-char Ed25519 seed (default: MGC_RELEASE_SIGNING_KEY)"
        )]
        key_hex: Option<String>,
    },
    #[command(about = "Read/write configuration (.npmrc)", alias = "c")]
    Config {
        #[command(subcommand)]
        cmd: crate::commands::config::ConfigCmd,
        #[arg(
            long,
            global = true,
            help = "read/write project .npmrc instead of ~/.npmrc"
        )]
        local: bool,
    },
    #[command(about = "Stage the package (pack + verify, no upload)")]
    Stage {
        #[arg(long, help = "stage a different project directory")]
        dir: Option<std::path::PathBuf>,
    },
    #[command(
        about = "Import legacy lockfile (package-lock.json, pnpm-lock.yaml, yarn.lock, bun.lock) to mgc.lock"
    )]
    Import {
        #[arg(long, help = "Target project directory to import")]
        dir: Option<std::path::PathBuf>,
        #[arg(
            long,
            help = "Allow writing an UNSIGNED mgc.lock when no signing key exists (explicit escape hatch — never the default)"
        )]
        allow_unsigned: bool,
    },
    #[command(about = "Migrate mgc.lock to a newer schema (explicit, never automatic)")]
    Migrate {
        #[command(subcommand)]
        cmd: crate::commands::migrate::MigrateCmd,
    },
    // ── W6: SBOM Export ──────────────────────────────────────────────
    #[command(about = "Generate Software Bill of Materials (SBOM) from lockfile")]
    Sbom {
        #[arg(
            long,
            help = "Output format (cyclonedx-json, spdx-json; SPDX uses version 2.3)"
        )]
        format: Option<String>,
        #[arg(long, help = "Output file path (default: stdout)")]
        output: Option<std::path::PathBuf>,
        #[arg(long, help = "Component name (default: project name)")]
        name: Option<String>,
        #[arg(long, help = "Component version (default: project version)")]
        version: Option<String>,
        #[arg(long, help = "Target project directory")]
        dir: Option<std::path::PathBuf>,
    },
    // ── Common: Publish ──────────────────────────────────────────────
    #[command(about = "Publish package to registry")]
    Publish {
        #[arg(long, value_parser = ["npm", "pypi", "oci"], default_value = "npm")]
        protocol: String,
        #[arg(long, requires = "trusted", help = "package name for PyPI or OCI")]
        package: Option<String>,
        #[arg(long, requires = "trusted", help = "version for PyPI or tag for OCI")]
        version: Option<String>,
        #[arg(
            long = "artifact",
            requires = "trusted",
            help = "PyPI or OCI artifact file (repeatable)"
        )]
        artifacts: Vec<std::path::PathBuf>,
        #[arg(
            long,
            requires = "trusted",
            help = "local Docker image reference for OCI image publish"
        )]
        image: Option<String>,
        #[arg(long, help = "dist-tag (default: latest)")]
        tag: Option<String>,
        #[arg(long, help = "access level: public|restricted")]
        access: Option<String>,
        #[arg(long, help = "pack + verify, do not publish")]
        dry_run: bool,
        #[arg(long, help = "output JSON")]
        json: bool,
        #[arg(long, help = "OTP code for 2FA")]
        otp: Option<String>,
        #[arg(long, help = "delete existing version then republish (409)")]
        force: bool,
        #[arg(long, help = "skip lifecycle scripts")]
        ignore_scripts: bool,
        #[arg(
            long,
            conflicts_with = "ignore_scripts",
            help = "explicitly allow package publish lifecycle scripts"
        )]
        allow_scripts: bool,
        #[arg(long, help = "skip git checks")]
        no_git_checks: bool,
        #[arg(
            long,
            help = "publish branch (default: current branch tracking remote)"
        )]
        publish_branch: Option<String>,
        #[arg(long, help = "all in one PUT (Phase 3)")]
        batch: bool,
        #[arg(long, help = "write JSON report file")]
        report_summary: bool,
        #[arg(long, help = "version bump patch")]
        patch: bool,
        #[arg(long, help = "version bump minor")]
        minor: bool,
        #[arg(long, help = "version bump major")]
        major: bool,
        #[arg(long, env = "MGC_REGISTRY", help = "override registry URL")]
        registry: Option<String>,
        #[arg(long, help = "override token (env MGC_NPM_TOKEN recommended)")]
        token: Option<String>,
        #[arg(
            long,
            conflicts_with = "token",
            help = "publish with a short-lived CI OIDC token"
        )]
        trusted: bool,
        #[arg(
            long,
            requires = "trusted",
            help = "OIDC audience (defaults to the registry origin)"
        )]
        trusted_audience: Option<String>,
    },
    // ── Common: Patch & Dedupe ───────────────────────────────────────
    #[command(about = "Manage package patches (add/rm/ls/verify)")]
    Patch {
        #[command(subcommand)]
        cmd: crate::commands::patch::PatchCmd,
    },
    #[command(about = "Deduplicate packages in lockfile (merge duplicates)")]
    Dedupe {
        #[arg(long, help = "report only, do not apply changes")]
        dry_run: bool,
        #[arg(long, help = "prefer latest version over existing instances")]
        prefer_latest: bool,
        #[arg(long, help = "output JSON")]
        json: bool,
    },
    // ── Registry (Phase 3) ──────────────────────────────────────────
    #[command(about = "Login to a registry (adduser flow, save token to .npmrc)")]
    Login {
        #[arg(long, help = "registry URL (default: config or npmjs)")]
        registry: Option<String>,
        #[arg(long, help = "username (prompt if omitted)")]
        username: Option<String>,
        #[arg(
            long,
            help = "Read the password from stdin; otherwise prompt without echo"
        )]
        password_stdin: bool,
        #[arg(long, help = "write token to project .npmrc instead of ~/.npmrc")]
        local: bool,
    },
    #[command(about = "Manage the private registry (serve, user add/rm)")]
    Registry {
        #[command(subcommand)]
        cmd: crate::commands::registry::RegistryCmd,
    },
    #[command(about = "Push/pull/list AI models qua OCI registry")]
    Model {
        #[command(subcommand)]
        cmd: crate::commands::model::ModelCmd,
    },
    #[command(about = "Start native Model Context Protocol (MCP) server for AI coding agents")]
    Mcp,
    #[command(about = "Show the capability manifest of each core (Global Gate 1)")]
    Capabilities {
        #[arg(long, help = "Explicitly request JSON output (the default format)")]
        json: bool,
    },
}
