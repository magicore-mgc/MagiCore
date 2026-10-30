// CreateCommands schema — khai báo nhóm lệnh CLI theo trách nhiệm.
use clap::Subcommand;

#[derive(Subcommand, Clone)]
#[allow(clippy::large_enum_variant)]
#[allow(clippy::enum_variant_names)] // Variant prefixes preserve the established create-<core> CLI naming.
pub enum CreateCommands {
    // ── Per-core: create-<core> ────────────────────────────────
    #[cfg_attr(not(feature = "web"), command(hide = true))]
    #[cfg_attr(
        all(
            feature = "web",
            not(any(
                feature = "game",
                feature = "ai",
                feature = "clo",
                feature = "cicd",
                feature = "iot",
                feature = "app",
                feature = "lib"
            ))
        ),
        command(visible_alias = "create")
    )]
    #[command(
        name = "create-web",
        about = "Scaffold a new web project",
        visible_alias = "cre-w"
    )]
    CreateWeb {
        /// Framework with optional version (e.g., react@latest, nextjs@14.0.0)
        #[arg(value_name = "FRAMEWORK[@VERSION]")]
        framework: String,
        /// Project directory name
        #[arg(value_name = "PROJECT")]
        project_name: String,
        #[command(flatten)]
        flags: crate::commands::web_scaffold_flags::WebScaffoldFlags,
    },
    #[command(
        name = "create-game",
        about = "Scaffold a new game project",
        visible_alias = "cre-g"
    )]
    CreateGame {
        /// Framework with optional version
        #[arg(value_name = "FRAMEWORK[@VERSION]")]
        framework: String,
        /// Project directory name
        #[arg(value_name = "PROJECT", default_value = "")]
        project_name: String,
        #[command(flatten)]
        source: crate::commands::core::shared::source_flags::SourceFlags,
        #[command(flatten)]
        automation: crate::commands::core::create::automation::CreateAutomationFlags,
    },
    #[command(
        name = "create-ai",
        about = "Scaffold a new AI project",
        visible_alias = "cre-ai"
    )]
    CreateAi {
        /// Framework with optional version
        #[arg(value_name = "FRAMEWORK[@VERSION]")]
        framework: String,
        /// Project directory name
        #[arg(value_name = "PROJECT", default_value = "")]
        project_name: String,
        #[command(flatten)]
        container: crate::commands::core::shared::container_flags::ContainerFlags,
        #[command(flatten)]
        source: crate::commands::core::shared::source_flags::SourceFlags,
        #[command(flatten)]
        automation: crate::commands::core::create::automation::CreateAutomationFlags,
    },
    #[command(
        name = "create-clo",
        about = "Scaffold a new cloud project",
        visible_alias = "cre-c"
    )]
    CreateClo {
        /// Framework with optional version
        #[arg(value_name = "FRAMEWORK[@VERSION]")]
        framework: String,
        /// Project directory name
        #[arg(value_name = "PROJECT", default_value = "")]
        project_name: String,
        #[command(flatten)]
        container: crate::commands::core::shared::container_flags::ContainerFlags,
        #[command(flatten)]
        source: crate::commands::core::shared::source_flags::SourceFlags,
        #[command(flatten)]
        automation: crate::commands::core::create::automation::CreateAutomationFlags,
    },
    #[command(
        name = "create-cicd",
        about = "Scaffold a new CI/CD project",
        visible_alias = "cre-ci"
    )]
    CreateCicd {
        /// Framework with optional version
        #[arg(value_name = "FRAMEWORK[@VERSION]")]
        framework: String,
        /// Project directory name
        #[arg(value_name = "PROJECT", default_value = "")]
        project_name: String,
        #[command(flatten)]
        source: crate::commands::core::shared::source_flags::SourceFlags,
        #[command(flatten)]
        automation: crate::commands::core::create::automation::CreateAutomationFlags,
    },
    #[command(
        name = "create-iot",
        about = "Scaffold a new IoT project",
        visible_alias = "cre-i"
    )]
    CreateIot {
        /// Framework with optional version
        #[arg(value_name = "FRAMEWORK[@VERSION]")]
        framework: String,
        /// Project directory name
        #[arg(value_name = "PROJECT", default_value = "")]
        project_name: String,
        #[command(flatten)]
        source: crate::commands::core::shared::source_flags::SourceFlags,
        #[command(flatten)]
        automation: crate::commands::core::create::automation::CreateAutomationFlags,
    },
    #[command(
        name = "create-app",
        about = "Scaffold a new app project",
        visible_alias = "cre-a"
    )]
    CreateApp {
        /// Framework with optional version
        #[arg(value_name = "FRAMEWORK[@VERSION]")]
        framework: String,
        /// Project directory name
        #[arg(value_name = "PROJECT", default_value = "")]
        project_name: String,
        #[command(flatten)]
        container: crate::commands::core::shared::container_flags::ContainerFlags,
        #[command(flatten)]
        source: crate::commands::core::shared::source_flags::SourceFlags,
        #[command(flatten)]
        automation: crate::commands::core::create::automation::CreateAutomationFlags,
    },
    #[command(
        name = "create-lib",
        about = "Scaffold a new library project",
        visible_alias = "cre-l"
    )]
    CreateLib {
        /// Language/framework with optional version
        #[arg(value_name = "FRAMEWORK[@VERSION]")]
        framework: String,
        /// Project directory name
        #[arg(value_name = "PROJECT", default_value = "")]
        project_name: String,
        #[command(flatten)]
        container: crate::commands::core::shared::container_flags::ContainerFlags,
        #[command(flatten)]
        source: crate::commands::core::shared::source_flags::SourceFlags,
        #[command(flatten)]
        automation: crate::commands::core::create::automation::CreateAutomationFlags,
    },
    #[command(
        name = "create-hardware",
        about = "Scaffold hardware packages (optimizer/bench)",
        visible_alias = "cre-h"
    )]
    CreateHardware {
        /// Framework with optional version
        #[arg(value_name = "FRAMEWORK[@VERSION]")]
        framework: String,
        /// Project directory name
        #[arg(value_name = "PROJECT", default_value = "")]
        project_name: String,
        #[command(flatten)]
        source: crate::commands::core::shared::source_flags::SourceFlags,
        #[command(flatten)]
        automation: crate::commands::core::create::automation::CreateAutomationFlags,
    },
}
