// UpdateCommands schema — khai báo nhóm lệnh CLI theo trách nhiệm.
use clap::Subcommand;

#[derive(Subcommand, Clone)]
#[allow(clippy::large_enum_variant)]
#[allow(clippy::enum_variant_names)] // Variant prefixes preserve the established update-<core> CLI naming.
pub enum UpdateCommands {
    // ── Per-core: update-<core> ────────────────────────────────
    #[cfg_attr(not(feature = "web"), command(hide = true))]
    #[command(name = "update-web", alias = "up-web", about = "Update web packages")]
    UpdateWeb {
        packages: Vec<String>,
        #[arg(long, help = "Install updated packages immediately")]
        install: bool,
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
        name = "update-game",
        alias = "up-game",
        about = "Update game packages"
    )]
    UpdateGame {
        packages: Vec<String>,
        #[arg(long, help = "Install updated packages immediately")]
        install: bool,
        /// Explicit toolchain-compat lane for delegated dependency
        /// operations (warned + audit-logged; excluded from
        /// native-support claims). Native lanes ignore it.
        #[arg(
            long,
            help = "Legacy flag; external package-manager execution is disabled"
        )]
        compat_runtime: Option<String>,
    },
    #[command(name = "update-ai", alias = "up-ai", about = "Update AI packages")]
    UpdateAi {
        packages: Vec<String>,
        #[arg(long, help = "Install updated packages immediately")]
        install: bool,
        /// Explicit toolchain-compat lane for delegated dependency
        /// operations (warned + audit-logged; excluded from
        /// native-support claims). Native lanes ignore it.
        #[arg(
            long,
            help = "Legacy flag; external package-manager execution is disabled"
        )]
        compat_runtime: Option<String>,
    },
    #[command(name = "update-clo", alias = "up-clo", about = "Update cloud packages")]
    UpdateClo {
        packages: Vec<String>,
        #[arg(long, help = "Install updated packages immediately")]
        install: bool,
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
        name = "update-cicd",
        alias = "up-cicd",
        about = "Update CI/CD packages"
    )]
    UpdateCicd {
        packages: Vec<String>,
        #[arg(long, help = "Install updated packages immediately")]
        install: bool,
        /// Explicit toolchain-compat lane for delegated dependency
        /// operations (warned + audit-logged; excluded from
        /// native-support claims). Native lanes ignore it.
        #[arg(
            long,
            help = "Legacy flag; external package-manager execution is disabled"
        )]
        compat_runtime: Option<String>,
    },
    #[command(name = "update-iot", alias = "up-iot", about = "Update IoT packages")]
    UpdateIot {
        packages: Vec<String>,
        #[arg(long, help = "Install updated packages immediately")]
        install: bool,
        /// Explicit toolchain-compat lane for delegated dependency
        /// operations (warned + audit-logged; excluded from
        /// native-support claims). Native lanes ignore it.
        #[arg(
            long,
            help = "Legacy flag; external package-manager execution is disabled"
        )]
        compat_runtime: Option<String>,
    },
    #[command(name = "update-app", alias = "up-app", about = "Update app packages")]
    UpdateApp {
        packages: Vec<String>,
        #[arg(long, help = "Install updated packages immediately")]
        install: bool,
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
        name = "update-lib",
        alias = "up-lib",
        about = "Update library packages"
    )]
    UpdateLib {
        packages: Vec<String>,
        #[arg(long, help = "Install updated packages immediately")]
        install: bool,
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
        name = "update-hardware",
        alias = "up-hardware",
        about = "Refresh materialized hardware templates (optimizer/bench)"
    )]
    UpdateHardware {
        packages: Vec<String>,
        #[arg(long, help = "Install updated packages immediately")]
        install: bool,
        /// Explicit toolchain-compat lane for delegated dependency
        /// operations (warned + audit-logged; excluded from
        /// native-support claims). Template lanes ignore it.
        #[arg(
            long,
            help = "Legacy flag; external package-manager execution is disabled"
        )]
        compat_runtime: Option<String>,
    },
}
