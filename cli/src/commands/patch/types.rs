//! Public Clap command arguments. / Đối số Clap public.

use clap::{Args, Subcommand};

#[derive(Args, Debug, Clone)]
pub struct PatchArgs {
    #[command(subcommand)]
    pub cmd: PatchCmd,
}

#[derive(Subcommand, Debug, Clone)]
pub enum PatchCmd {
    /// Add a patch for a package
    Add {
        #[arg(required = true, help = "Package name (e.g. react, @scope/pkg)")]
        package: String,
        #[arg(long, short = 'f', help = "Patch file (unified diff)")]
        file: String,
        #[arg(long, help = "Version range (e.g. 1.0.0 - 1.2.0)")]
        range: Option<String>,
    },
    /// Remove a patch for a package
    Remove {
        #[arg(required = true, help = "Package name")]
        package: String,
    },
    /// List active patches
    List,
    /// Verify patch integrity against lockfile
    Verify,
}
