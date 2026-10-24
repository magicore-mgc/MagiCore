//! Offline scaffold source flags.
//! Cờ nguồn scaffold offline.
use std::path::PathBuf;

use clap::Args;
use serde::{Deserialize, Serialize};

/// Where a scaffold may take framework source from when the registry layer is
/// unavailable. MagiCore never synthesizes framework source itself.
/// Nguồn source framework được phép dùng khi registry layer không sẵn sàng.
/// MagiCore không bao giờ tự sinh source framework.
#[derive(Debug, Clone, Args, Serialize, Deserialize, Default)]
#[command(next_help_heading = "Offline source")]
pub struct SourceFlags {
    #[arg(
        long,
        value_name = "PATH",
        help = "Reuse an existing project with the same framework as the scaffold source"
    )]
    pub from_local: Option<PathBuf>,
    #[arg(
        long,
        help = "Do not search this machine for a reusable project; require the registry layer"
    )]
    pub no_local: bool,
}
