// AddCommands schema — khai báo nhóm lệnh CLI theo trách nhiệm.
use clap::Subcommand;

#[derive(Subcommand, Clone)]
#[allow(clippy::large_enum_variant)]
#[allow(clippy::enum_variant_names)] // Variant prefixes preserve the established add-<core> CLI naming.
pub enum AddCommands {
    // ── Per-core: add-<core> ───────────────────────────────────
    #[cfg_attr(not(feature = "web"), command(hide = true))]
    #[command(name = "add-web", alias = "a-web", about = "Add web dependencies")]
    AddWeb {
        #[arg(required = true)]
        packages: Vec<String>,
        #[arg(short = 'D', long)]
        dev: bool,
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
        #[arg(short = 'g', long)]
        global: bool,
        #[arg(
            long,
            help = "Pin version for all listed packages (lanes without version semantics fail loudly; no short flag: -v is the global version alias)"
        )]
        version: Option<String>,
        /// Explicit toolchain-compat lane for delegated dependency
        /// operations (warned + audit-logged; excluded from
        /// native-support claims). Native lanes ignore it.
        #[arg(
            long,
            help = "Legacy flag; external package-manager execution is disabled"
        )]
        compat_runtime: Option<String>,
    },
    #[command(name = "add-game", alias = "a-game", about = "Add game dependencies")]
    AddGame {
        #[arg(required = true)]
        packages: Vec<String>,
        #[arg(short = 'D', long)]
        dev: bool,
        #[arg(short = 'E', long)]
        exact: bool,
        #[arg(short = 'O', long)]
        optional: bool,
        #[arg(short = 'P', long)]
        peer: bool,
        #[arg(long)]
        no_save: bool,
        #[arg(short = 'g', long)]
        global: bool,
        #[arg(
            long,
            help = "Pin version for all listed packages (lanes without version semantics fail loudly; no short flag: -v is the global version alias)"
        )]
        version: Option<String>,
        /// Explicit toolchain-compat lane for delegated dependency
        /// operations (warned + audit-logged; excluded from
        /// native-support claims). Native lanes ignore it.
        #[arg(
            long,
            help = "Legacy flag; external package-manager execution is disabled"
        )]
        compat_runtime: Option<String>,
    },
    #[command(name = "add-ai", alias = "a-ai", about = "Add AI dependencies")]
    AddAi {
        #[arg(required = true)]
        packages: Vec<String>,
        #[arg(short = 'D', long)]
        dev: bool,
        #[arg(short = 'E', long)]
        exact: bool,
        #[arg(short = 'O', long)]
        optional: bool,
        #[arg(short = 'P', long)]
        peer: bool,
        #[arg(long)]
        no_save: bool,
        #[arg(short = 'g', long)]
        global: bool,
        #[arg(
            long,
            help = "Pin version for all listed packages (lanes without version semantics fail loudly; no short flag: -v is the global version alias)"
        )]
        version: Option<String>,
        /// Explicit toolchain-compat lane for delegated dependency
        /// operations (warned + audit-logged; excluded from
        /// native-support claims). Native lanes ignore it.
        #[arg(
            long,
            help = "Legacy flag; external package-manager execution is disabled"
        )]
        compat_runtime: Option<String>,
    },
    #[command(name = "add-clo", alias = "a-clo", about = "Add cloud dependencies")]
    AddClo {
        #[arg(required = true)]
        packages: Vec<String>,
        #[arg(short = 'D', long)]
        dev: bool,
        #[arg(short = 'E', long)]
        exact: bool,
        #[arg(short = 'O', long)]
        optional: bool,
        #[arg(short = 'P', long)]
        peer: bool,
        #[arg(long)]
        no_save: bool,
        #[arg(short = 'g', long)]
        global: bool,
        #[arg(
            long,
            help = "Pin version for all listed packages (lanes without version semantics fail loudly; no short flag: -v is the global version alias)"
        )]
        version: Option<String>,
        /// Explicit toolchain-compat lane for delegated dependency
        /// operations (warned + audit-logged; excluded from
        /// native-support claims). Native lanes ignore it.
        #[arg(
            long,
            help = "Legacy flag; external package-manager execution is disabled"
        )]
        compat_runtime: Option<String>,
    },
    #[command(name = "add-cicd", alias = "a-cicd", about = "Add CI/CD dependencies")]
    AddCicd {
        #[arg(required = true)]
        packages: Vec<String>,
        #[arg(short = 'D', long)]
        dev: bool,
        #[arg(short = 'E', long)]
        exact: bool,
        #[arg(short = 'O', long)]
        optional: bool,
        #[arg(short = 'P', long)]
        peer: bool,
        #[arg(long)]
        no_save: bool,
        #[arg(short = 'g', long)]
        global: bool,
        #[arg(
            long,
            help = "Pin version for all listed packages (lanes without version semantics fail loudly; no short flag: -v is the global version alias)"
        )]
        version: Option<String>,
        /// Explicit toolchain-compat lane for delegated dependency
        /// operations (warned + audit-logged; excluded from
        /// native-support claims). Native lanes ignore it.
        #[arg(
            long,
            help = "Legacy flag; external package-manager execution is disabled"
        )]
        compat_runtime: Option<String>,
    },
    #[command(name = "add-iot", alias = "a-iot", about = "Add IoT dependencies")]
    AddIot {
        #[arg(required = true)]
        packages: Vec<String>,
        #[arg(short = 'D', long)]
        dev: bool,
        #[arg(short = 'E', long)]
        exact: bool,
        #[arg(short = 'O', long)]
        optional: bool,
        #[arg(short = 'P', long)]
        peer: bool,
        #[arg(long)]
        no_save: bool,
        #[arg(short = 'g', long)]
        global: bool,
        #[arg(
            long,
            help = "Pin version for all listed packages (lanes without version semantics fail loudly; no short flag: -v is the global version alias)"
        )]
        version: Option<String>,
        /// Explicit toolchain-compat lane for delegated dependency
        /// operations (warned + audit-logged; excluded from
        /// native-support claims). Native lanes ignore it.
        #[arg(
            long,
            help = "Legacy flag; external package-manager execution is disabled"
        )]
        compat_runtime: Option<String>,
    },
    #[command(name = "add-app", alias = "a-app", about = "Add app dependencies")]
    AddApp {
        #[arg(required = true)]
        packages: Vec<String>,
        #[arg(short = 'D', long)]
        dev: bool,
        #[arg(short = 'E', long)]
        exact: bool,
        #[arg(short = 'O', long)]
        optional: bool,
        #[arg(short = 'P', long)]
        peer: bool,
        #[arg(long)]
        no_save: bool,
        #[arg(short = 'g', long)]
        global: bool,
        #[arg(
            long,
            help = "Pin version for all listed packages (lanes without version semantics fail loudly; no short flag: -v is the global version alias)"
        )]
        version: Option<String>,
        /// Explicit toolchain-compat lane for delegated dependency
        /// operations (warned + audit-logged; excluded from
        /// native-support claims). Native lanes ignore it.
        #[arg(
            long,
            help = "Legacy flag; external package-manager execution is disabled"
        )]
        compat_runtime: Option<String>,
    },
    #[command(name = "add-lib", alias = "a-lib", about = "Add library dependencies")]
    AddLib {
        #[arg(required = true)]
        packages: Vec<String>,
        #[arg(short = 'D', long)]
        dev: bool,
        #[arg(short = 'E', long)]
        exact: bool,
        #[arg(short = 'O', long)]
        optional: bool,
        #[arg(short = 'P', long)]
        peer: bool,
        #[arg(long)]
        no_save: bool,
        #[arg(short = 'g', long)]
        global: bool,
        #[arg(
            long,
            help = "Pin version for all listed packages (lanes without version semantics fail loudly; no short flag: -v is the global version alias)"
        )]
        version: Option<String>,
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
        name = "add-hardware",
        alias = "a-hardware",
        about = "Add hardware packages (optimizer/bench)"
    )]
    AddHardware {
        #[arg(required = true)]
        packages: Vec<String>,
        #[arg(
            long,
            help = "Pin version for all listed packages (lanes without version semantics fail loudly; no short flag: -v is the global version alias)"
        )]
        version: Option<String>,
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

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
