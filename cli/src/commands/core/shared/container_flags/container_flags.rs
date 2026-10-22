//! Shared Docker/Kubernetes scaffold flags.
//! Cờ scaffold Docker/Kubernetes dùng chung.
use clap::Args;
use serde::{Deserialize, Serialize};

/// Container lifecycle flags shared by non-Web scaffold commands.
/// Các cờ vòng đời container dùng chung cho lệnh scaffold ngoài Web.
#[derive(Debug, Clone, Args, Serialize, Deserialize, Default)]
#[command(next_help_heading = "Container")]
pub struct ContainerFlags {
    #[arg(long, help = "Generate a Docker image that runs the MGC lifecycle")]
    pub docker: bool,
    #[arg(long, help = "Generate a Kubernetes deployment profile when supported")]
    pub k8s: bool,
}
