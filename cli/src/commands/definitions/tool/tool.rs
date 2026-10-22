// ToolCmd argument schema — khai báo tham số cho tool lifecycle.
use clap::Subcommand;

#[derive(Subcommand, Clone)]
pub enum ToolCmd {
    #[command(about = "Install a CLI tool globally (currently npm:<package>)")]
    Install {
        /// Explicit package source and package name, for example npm:@openai/codex@latest.
        package: String,
        #[arg(long, value_name = "NAME", help = "Select one declared executable")]
        bin: Option<String>,
        #[arg(long, help = "Allow package lifecycle scripts during installation")]
        allow_scripts: bool,
    },
    #[command(about = "Update an installed CLI tool")]
    Update {
        /// Installed executable name shown by `mgc tool list`.
        binary: String,
        #[arg(
            long,
            value_name = "VERSION",
            help = "Version, range, or dist-tag (default: latest)"
        )]
        version: Option<String>,
        #[arg(long, help = "Allow package lifecycle scripts during installation")]
        allow_scripts: bool,
    },
    #[command(about = "Remove an installed CLI tool")]
    Remove {
        /// Installed executable name shown by `mgc tool list`.
        binary: String,
    },
    #[command(about = "List persistent user-scoped CLI tools")]
    List,
}
