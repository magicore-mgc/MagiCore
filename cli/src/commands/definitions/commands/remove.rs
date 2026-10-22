// RemoveCommands schema — khai báo nhóm lệnh CLI theo trách nhiệm.
use clap::Subcommand;

#[derive(Subcommand, Clone)]
#[allow(clippy::large_enum_variant)]
#[allow(clippy::enum_variant_names)] // Variant prefixes preserve the established remove-<core> CLI naming.
pub enum RemoveCommands {
    // ── Per-core: remove-<core> ────────────────────────────────
    #[cfg_attr(not(feature = "web"), command(hide = true))]
    #[command(
        name = "remove-web",
        alias = "rm-web",
        about = "Remove web dependencies"
    )]
    RemoveWeb {
        #[arg(required = true)]
        packages: Vec<String>,
        #[arg(long, help = "Only update manifest, do not reinstall dependencies")]
        no_install: bool,
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
        name = "remove-game",
        alias = "rm-game",
        about = "Remove game dependencies"
    )]
    RemoveGame {
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
    #[command(name = "remove-ai", alias = "rm-ai", about = "Remove AI dependencies")]
    RemoveAi {
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
        name = "remove-clo",
        alias = "rm-clo",
        about = "Remove cloud dependencies"
    )]
    RemoveClo {
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
        name = "remove-cicd",
        alias = "rm-cicd",
        about = "Remove CI/CD dependencies"
    )]
    RemoveCicd {
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
        name = "remove-iot",
        alias = "rm-iot",
        about = "Remove IoT dependencies"
    )]
    RemoveIot {
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
        name = "remove-app",
        alias = "rm-app",
        about = "Remove app dependencies"
    )]
    RemoveApp {
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
        name = "remove-lib",
        alias = "rm-lib",
        about = "Remove library dependencies"
    )]
    RemoveLib {
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
        name = "remove-hardware",
        alias = "rm-hardware",
        about = "Remove materialized hardware templates (optimizer/bench)"
    )]
    RemoveHardware {
        packages: Vec<String>,
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
