// InstallCommands schema — khai báo nhóm lệnh CLI theo trách nhiệm.
use clap::Subcommand;

#[derive(Subcommand, Clone)]
#[allow(clippy::large_enum_variant)]
#[allow(clippy::enum_variant_names)] // Variant prefixes preserve the established install-<core> CLI naming.
pub enum InstallCommands {
    /// Install a signed binary into the lib keg store (E-3).
    /// Cài binary có chữ ký vào keg store của core lib (E-3).
    #[cfg_attr(not(feature = "lib"), command(hide = true))]
    // `install-lib` is the dependency command; this binary transfer lane needs
    // its own name and derives the `i-lib` alias (the spec's spelling).
    // (`install-lib` là lệnh dependency; làn chuyển binary cần tên riêng và
    // sinh alias `i-lib`, đúng cách viết trong spec.)
    #[command(
        name = "install-lib-binary",
        alias = "i-lib",
        about = "Install a signed binary into the lib keg store"
    )]
    InstallLibBinary {
        /// Binary name, existing path, or `name@version` for a manifest entry.
        /// Tên binary, đường dẫn có sẵn, hoặc `name@version` cho entry manifest.
        target: String,
        /// Signed manifest describing the artifact.
        /// Manifest có chữ ký mô tả artifact.
        #[arg(long, value_name = "PATH")]
        manifest: Option<String>,
        /// Keyring key id used to sign the manifest.
        /// Key id trong keyring dùng để ký manifest.
        #[arg(long, value_name = "KEY_ID")]
        sign_key: Option<String>,
        /// Pinned Ed25519 public key (base64) allowed to sign manifests.
        /// Public key Ed25519 đã pin (base64) được phép ký manifest.
        #[arg(long = "trust-key", value_name = "BASE64")]
        trust_key: Vec<String>,
    },
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
    #[command(name = "install-lib", about = "Install library dependencies")]
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
