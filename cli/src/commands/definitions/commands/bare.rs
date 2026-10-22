// BareCommands schema — khai báo nhóm lệnh CLI theo trách nhiệm.
use clap::Subcommand;

#[derive(Subcommand, Clone)]
#[allow(clippy::large_enum_variant)]
pub enum BareCommands {
    // ── Bare Commands (In-project, auto-detect core from signature) ──
    #[command(about = "Install dependencies (auto-detect core)", alias = "i")]
    Install {
        packages: Vec<String>,
        #[arg(long, help = "Fail if mgc.lock is missing or outdated (CI mode)")]
        frozen: bool,
        #[arg(long, help = "Skip running lifecycle scripts")]
        ignore_scripts: bool,
        #[arg(long, help = "Allow dependency lifecycle scripts")]
        allow_scripts: bool,
        #[arg(
            long,
            help = "Prefer reusing installed versions instead of latest (dedupe, opt-in)"
        )]
        prefer_dedupe: bool,
        #[arg(long, help = "Re-link dangling symlinks in node_modules")]
        repair: bool,
        #[arg(
            long,
            help = "Preview install for supported cores; unsupported cores fail without changes"
        )]
        dry_run: bool,
        #[arg(
            long,
            help = "Offline mode: install from cache only, no network (T4.1)"
        )]
        offline: bool,
        /// Explicit toolchain-compat lane for delegated dependency
        /// operations (warned + audit-logged; excluded from
        /// native-support claims).
        #[arg(
            long,
            help = "Legacy flag; external package-manager execution is disabled"
        )]
        compat_runtime: Option<String>,
    },
    #[command(about = "Check managed store routes and maintain the Web package store")]
    Store {
        #[command(subcommand)]
        cmd: crate::commands::store::StoreCmd,
    },
    #[command(about = "Benchmark install (phases + wall-time, C3)")]
    Bench {
        #[command(flatten)]
        args: crate::commands::bench::BenchArgs,
    },
    #[command(about = "Approve/deny lifecycle scripts per package (T5 trust gate)")]
    Trust {
        #[command(subcommand)]
        cmd: crate::commands::trust::TrustCmd,
    },
    #[command(about = "Run user-defined pre/post scripts (mgc.hooks.toml)")]
    Hooks {
        #[command(subcommand)]
        cmd: crate::commands::hooks::HooksCmd,
    },
    #[command(about = "Auto-generate CLI docs from clap schema")]
    Docs {
        #[arg(short, long, help = "write docs to file (default: stdout)")]
        output: Option<std::path::PathBuf>,
    },
    #[command(about = "Print shell completions (bash/zsh/fish/powershell/elvish)")]
    Completion {
        #[arg(help = "shell to generate completions for")]
        shell: crate::commands::completion::CompletionShell,
    },
    #[command(about = "Telemetry opt-in status/log (default OFF — sends nothing)")]
    Telemetry {
        #[command(subcommand)]
        cmd: crate::commands::telemetry::TelemetryCmd,
    },
    #[command(about = "Network transparency — list EVERY outbound connection + reachability")]
    Network {
        #[command(subcommand)]
        cmd: crate::commands::network::NetworkCmd,
    },
    #[command(about = "Environment diagnostic (toolchain, store, disk, network)")]
    Doctor {
        #[command(subcommand)]
        cmd: crate::commands::doctor::DoctorCmd,
    },
    #[command(about = "Manage kernel templates (publish/fetch — registry-backed)")]
    Template {
        #[command(subcommand)]
        cmd: crate::commands::template::TemplateCmd,
    },
    #[command(about = "Show workspace graph (nodes + workspace:* edges)")]
    Workspace {
        #[command(subcommand)]
        cmd: crate::commands::workspace::WorkspaceCmd,
    },
    #[command(about = "Add dependencies (auto-detect core)")]
    Add {
        #[arg(required = true)]
        packages: Vec<String>,
        #[arg(short, long)]
        version: Option<String>,
        #[arg(short = 'D', long)]
        dev: bool,
        #[arg(short = 'g', long)]
        global: bool,
        #[arg(short = 'E', long)]
        exact: bool,
        #[arg(short = 'O', long)]
        optional: bool,
        #[arg(short = 'P', long)]
        peer: bool,
        #[arg(long)]
        no_save: bool,
        #[arg(long, help = "Only update manifest, do not install dependencies")]
        no_install: bool,
        /// Explicit toolchain-compat lane for delegated dependency
        /// operations (warned + audit-logged; excluded from
        /// native-support claims).
        #[arg(
            long,
            help = "Legacy flag; external package-manager execution is disabled"
        )]
        compat_runtime: Option<String>,
    },
    #[command(about = "Remove dependencies (auto-detect core)", alias = "rm")]
    Remove {
        #[arg(required = true)]
        packages: Vec<String>,
        #[arg(long, help = "Only update manifest, do not reinstall dependencies")]
        no_install: bool,
        /// Explicit toolchain-compat lane for delegated dependency
        /// operations (warned + audit-logged; excluded from
        /// native-support claims).
        #[arg(
            long,
            help = "Legacy flag; external package-manager execution is disabled"
        )]
        compat_runtime: Option<String>,
    },
    #[command(about = "Update packages (auto-detect core)", alias = "up")]
    Update {
        packages: Vec<String>,
        #[arg(long, help = "Install updated packages immediately")]
        install: bool,
        /// Explicit toolchain-compat lane for delegated dependency
        /// operations (warned + audit-logged; excluded from
        /// native-support claims).
        #[arg(
            long,
            help = "Legacy flag; external package-manager execution is disabled"
        )]
        compat_runtime: Option<String>,
    },
    #[command(about = "List installed packages (auto-detect core)", alias = "ls")]
    List {
        /// Explicit toolchain-compat lane for delegated dependency
        /// operations (warned + audit-logged; excluded from
        /// native-support claims). Native lanes ignore it.
        #[arg(
            long,
            help = "Legacy flag; external package-manager execution is disabled"
        )]
        compat_runtime: Option<String>,
    },
    #[command(about = "Connect the local project to another one", alias = "ln")]
    Link { package: Option<String> },
    #[command(about = "Unlinks a package")]
    Unlink { package: Option<String> },
    #[command(about = "Shows all packages that depend on the specified package")]
    Why { package: String },
}
