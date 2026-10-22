// InstallCommands schema — khai báo nhóm lệnh CLI theo trách nhiệm.
use clap::Subcommand;

#[derive(Subcommand, Clone)]
#[allow(clippy::large_enum_variant)]
#[allow(clippy::enum_variant_names)] // Variant prefixes preserve the established install-<core> CLI naming.
pub enum InstallCommands {
    // ── Per-core: install-<core> ───────────────────────────────────
    #[cfg_attr(not(feature = "web"), command(hide = true))]
    #[command(
        name = "install-web",
        alias = "i-web",
        about = "Install web dependencies"
    )]
    InstallWeb {
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
        #[arg(long, help = "Use only cached packages, no network requests")]
        offline: bool,
        /// Explicit toolchain-compat lane for delegated dependency
        /// operations (warned + audit-logged; excluded from
        /// native-support claims). Native lanes ignore it.
        #[arg(
            long,
            help = "Legacy flag; external package-manager execution is disabled"
        )]
        compat_runtime: Option<String>,
    },
    #[command(
        name = "install-game",
        alias = "i-game",
        about = "Install game dependencies"
    )]
    InstallGame {
        packages: Vec<String>,
        #[arg(long, help = "Fail if the lockfile is missing or stale")]
        frozen: bool,
        /// Explicit toolchain-compat lane for delegated dependency
        /// operations (warned + audit-logged; excluded from
        /// native-support claims). Native lanes ignore it.
        #[arg(
            long,
            help = "Legacy flag; external package-manager execution is disabled"
        )]
        compat_runtime: Option<String>,
    },
    #[command(name = "install-ai", alias = "i-ai", about = "Install AI dependencies")]
    InstallAi {
        packages: Vec<String>,
        #[arg(long)]
        dry_run: bool,
        /// Explicit toolchain-compat lane for delegated dependency
        /// operations (warned + audit-logged; excluded from
        /// native-support claims). Native lanes ignore it.
        #[arg(
            long,
            help = "Legacy flag; external package-manager execution is disabled"
        )]
        compat_runtime: Option<String>,
        /// Frozen mode: fail if mgc.lock is missing or does not match
        /// (never silently re-resolve).
        #[arg(long, help = "Fail if the lockfile is missing or stale")]
        frozen: bool,
        /// Replay verified cached artifacts without registry access.
        /// Replay artifact cache đã xác minh, không truy cập registry.
        #[arg(long)]
        offline: bool,
    },
    #[command(
        name = "install-clo",
        alias = "i-clo",
        about = "Install cloud dependencies"
    )]
    InstallClo {
        packages: Vec<String>,
        #[arg(long, help = "Fail if the lockfile is missing or stale")]
        frozen: bool,
        /// Explicit toolchain-compat lane for delegated dependency
        /// operations (warned + audit-logged; excluded from
        /// native-support claims). Native lanes ignore it.
        #[arg(
            long,
            help = "Legacy flag; external package-manager execution is disabled"
        )]
        compat_runtime: Option<String>,
    },
    #[command(
        name = "install-cicd",
        alias = "i-cicd",
        about = "Install CI/CD dependencies"
    )]
    InstallCicd {
        packages: Vec<String>,
        /// Explicit toolchain-compat lane for delegated dependency
        /// operations (warned + audit-logged; excluded from
        /// native-support claims). Native lanes ignore it.
        #[arg(
            long,
            help = "Legacy flag; external package-manager execution is disabled"
        )]
        compat_runtime: Option<String>,
    },
    #[command(
        name = "install-iot",
        alias = "i-iot",
        about = "Install IoT dependencies"
    )]
    InstallIot {
        packages: Vec<String>,
        #[arg(long, help = "Fail if the lockfile is missing or stale")]
        frozen: bool,
        /// Explicit toolchain-compat lane for delegated dependency
        /// operations (warned + audit-logged; excluded from
        /// native-support claims). Native lanes ignore it.
        #[arg(
            long,
            help = "Legacy flag; external package-manager execution is disabled"
        )]
        compat_runtime: Option<String>,
    },
    #[command(
        name = "install-app",
        alias = "i-app",
        about = "Install app dependencies"
    )]
    InstallApp {
        packages: Vec<String>,
        /// Explicit toolchain-compat lane for delegated dependency
        /// operations (warned + audit-logged; excluded from
        /// native-support claims). Native lanes ignore it.
        #[arg(
            long,
            help = "Legacy flag; external package-manager execution is disabled"
        )]
        compat_runtime: Option<String>,
        /// Frozen mode: fail if mgc.lock is missing or does not match
        /// (never silently re-resolve).
        #[arg(long, help = "Fail if the lockfile is missing or stale")]
        frozen: bool,
        /// Replay verified Flutter archives without registry access.
        /// Replay archive Flutter đã verify, không dùng registry.
        #[arg(long)]
        offline: bool,
    },
    #[command(
        name = "install-lib",
        alias = "i-lib",
        about = "Install library dependencies"
    )]
    InstallLib {
        packages: Vec<String>,
        /// Explicit toolchain-compat lane for delegated dependency
        /// operations (warned + audit-logged; excluded from
        /// native-support claims). Native lanes ignore it.
        #[arg(
            long,
            help = "Legacy flag; external package-manager execution is disabled"
        )]
        compat_runtime: Option<String>,
        /// Frozen mode: fail if mgc.lock is missing or does not match
        /// (never silently re-resolve).
        #[arg(long, help = "Fail if the lockfile is missing or stale")]
        frozen: bool,
        #[arg(long, help = "Offline mode: install from verified MGC cache only")]
        offline: bool,
    },
    #[command(
        name = "install-hardware",
        alias = "i-hardware",
        about = "Install hardware packages (optimizer/bench)"
    )]
    InstallHardware {
        packages: Vec<String>,
        /// Explicit toolchain-compat lane for delegated dependency
        /// operations (warned + audit-logged; excluded from
        /// native-support claims). Native lanes ignore it.
        #[arg(
            long,
            help = "Legacy flag; external package-manager execution is disabled"
        )]
        compat_runtime: Option<String>,
    },
}
