use super::implementation::{
    delete_npmrc, delete_toml, get, list, npmrc_path, set_npmrc, set_toml,
};
use anyhow::Result;
use clap::Subcommand;

/// Sub-commands for `mgc config` (help text must stay English — RULE §7).
/// Sub-commands cho `mgc config` (text help phải giữ tiếng Anh — RULE §7).
#[derive(Subcommand, Debug, Clone)]
pub enum ConfigCmd {
    /// Get a key value (priority: env → mgc.toml → local .npmrc → user .npmrc)
    Get {
        /// Key to read
        key: String,
    },
    /// Write a key=value pair to the config file
    Set {
        /// Key to set
        key: String,
        /// Value to set
        value: String,
        /// Write to mgc.toml instead of .npmrc
        #[arg(long, help = "write to mgc.toml instead of .npmrc")]
        toml: bool,
    },
    /// Remove a key from the config file
    Delete {
        /// Key to remove
        key: String,
        /// Remove from mgc.toml instead of .npmrc
        #[arg(long, help = "remove from mgc.toml instead of .npmrc")]
        toml: bool,
    },
    /// Alias for delete
    Unset {
        /// Key to remove
        key: String,
        /// Remove from mgc.toml instead of .npmrc
        #[arg(long, help = "remove from mgc.toml instead of .npmrc")]
        toml: bool,
    },
    /// List all config entries (shows the source of each)
    List {
        /// Only show project-local config (.npmrc local + mgc.toml)
        #[arg(long, help = "only show project-local config")]
        local: bool,
    },
}

/// Entry point — được gọi từ dispatch/common.rs
pub async fn run(cmd: ConfigCmd, global_local: bool) -> Result<()> {
    match cmd {
        ConfigCmd::Get { key } => get(&key),
        ConfigCmd::Set { key, value, toml } => {
            if toml {
                set_toml(&key, &value)
            } else {
                let path = npmrc_path(global_local)?;
                set_npmrc(&path, &key, &value)
            }
        }
        ConfigCmd::Delete { key, toml } | ConfigCmd::Unset { key, toml } => {
            if toml {
                delete_toml(&key)
            } else {
                let path = npmrc_path(global_local)?;
                delete_npmrc(&path, &key)
            }
        }
        ConfigCmd::List { local } => list(local || global_local),
    }
}
